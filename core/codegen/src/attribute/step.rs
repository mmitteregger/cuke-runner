use devise::{FromMeta, Result, Spanned, SpanWrapped, Diagnostic};
use devise::ext::SpanDiagnosticExt;
use syn::{Attribute, parse::Parser};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};

use crate::{STEP_FN_LOCATION_FN_PREFIX, STEP_FN_PREFIX, STEP_STRUCT_PREFIX};
use crate::attribute::GlueFnArg;
use crate::glue_codegen::{Regex, StepKeyword};
use crate::proc_macro_ext::{Diagnostics, StringLit};
use crate::syn_ext::IdentExt;

/// The raw, parsed `#[step]` attribute.
#[derive(Debug, FromMeta)]
struct StepAttribute {
    #[meta(naked)]
    keyword: SpanWrapped<StepKeyword>,
    expression: SpanWrapped<Regex>,
}

/// The raw, parsed `#[step]` (e.g, `given`, `when`, `then`) attribute.
#[derive(Debug, FromMeta)]
struct KeywordStepAttribute {
    #[meta(naked)]
    expression: SpanWrapped<Regex>,
}

/// This structure represents the parsed `step` attribute and associated items.
#[derive(Debug)]
struct Step {
    /// The status associated with the code in the `#[step(code)]` attribute.
    attribute: StepAttribute,
    /// The function that was decorated with the `step` attribute.
    function: syn::ItemFn,
    /// Parsed function arguments.
    arguments: Vec<GlueFnArg>,
}

fn parse_step(attr: StepAttribute, mut function: syn::ItemFn) -> Result<Step> {
    // Gather diagnostics as we proceed.
    let mut diags = Diagnostics::new();

    let arguments = super::parse_glue_fn_args(&mut diags, &mut function);

    diags.head_err_or(Step { attribute: attr, function, arguments })
}

fn step_data_expr(ident: &syn::Ident, ty: &syn::Type, step_argument_index: usize) -> TokenStream {
    let span = ident.span().join(ty.span()).unwrap_or_else(|| ty.span());

    if let syn::Type::Reference(ref type_reference) = ty {
        if let syn::Type::Path(ref type_path) = *type_reference.elem {
            if type_path.path.is_ident("str") {
                return quote_spanned! { span =>
                    #[allow(non_snake_case, unreachable_patterns)]
                    let #ident: #ty = {
                        use ::cuke_runner::glue::step::argument::StepArgument::*;

                        let str_value = match __step_arguments[#step_argument_index] {
                            Expression(ref expression) => Some(expression.value()),
                            DocString(ref doc_string) => Some(doc_string.value()),
                            DataTable(ref _data_table) => None,
                        };

                        match str_value {
                            Some(value) => value,
                            None => return Err(::cuke_runner::glue::error::ExecutionError::from(
                                ::cuke_runner::glue::step::argument::FromStepArgumentError::new(
                                    format!("cannot get str value from DataTable")
                                )
                            )),
                        }
                    };
                };
            }
        }
    }

    quote_spanned! { span =>
        #[allow(non_snake_case, unreachable_patterns)]
        let #ident: #ty = match ::cuke_runner::glue::step::argument::FromStepArgument::from_step_argument(
                &__step_arguments[#step_argument_index]
        ) {
            Ok(step_argument) => step_argument,
            Err(error) => {
                return Err(::cuke_runner::glue::error::ExecutionError::from(error))
            },
        };
    }
}

fn codegen_step(step: Step) -> Result<TokenStream> {
    // Gather everything we need.
    let (vis, user_handler_fn) = (&step.function.vis, &step.function);
    let user_handler_fn_name = &user_handler_fn.sig.ident;
    let user_handler_fn_span = user_handler_fn.sig.ident.span();
    let generated_location_fn_name = user_handler_fn_name.prepend(STEP_FN_LOCATION_FN_PREFIX);
    let generated_fn_name = user_handler_fn_name.prepend(STEP_FN_PREFIX);
    let generated_struct_name = user_handler_fn_name.prepend(STEP_STRUCT_PREFIX);
    let parameter_names = step.arguments.iter().map(|argument| &argument.cuke_runner_ident);
    let keyword = step.attribute.keyword;
    let expression = step.attribute.expression;

    let mut step_argument_index = 0;
    let mut data_statements = Vec::with_capacity(step.arguments.len());
    for argument in &step.arguments {
        let data_statement = if argument.scenario_arg {
            super::scenario_data_expr(&argument)?
        } else {
            let step_data_expr = step_data_expr(&argument.cuke_runner_ident, &argument.ty, step_argument_index);
            step_argument_index += 1;
            step_data_expr
        };

        data_statements.push(data_statement);
    }

    // quote_spanned so that the line information points
    // to the user handler function in the source file
    // instead of the macro invocation line
    let user_handler_fn_location_fn = quote_spanned! {user_handler_fn_span=>
        #[track_caller]
        #vis fn #generated_location_fn_name() -> ::cuke_runner::glue::location::StaticGlueCodeLocation {
            let location = std::panic::Location::caller();
            ::cuke_runner::glue::location::StaticGlueCodeLocation {
                file: location.file(),
                line: location.line(),
            }
        }
    };

    Ok(quote! {
        #[inline(never)] // to see the function in the stack trace in case of a panic
        #user_handler_fn

        #user_handler_fn_location_fn

        /// Cuke runner code generated wrapping step function.
        #vis fn #generated_fn_name(
            __scenario: &mut ::cuke_runner::glue::scenario::Scenario,
            __step_arguments: &[::cuke_runner::glue::step::argument::StepArgument],
        ) -> ::std::result::Result<(), ::cuke_runner::glue::error::ExecutionError> {

            #(#data_statements)*

            // TODO: error handling...
            let result = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| #user_handler_fn_name(#(#parameter_names),*)));
            match result {
                Ok(user_handler_fn_result) => return Ok(()),
                Err(_err) => return Err(::cuke_runner::glue::error::ExecutionError::Panic(::cuke_runner::glue::error::PanicError::new())),
            };
        }

        /// Cuke runner code generated static step info.
        #[allow(non_upper_case_globals)]
        #vis static #generated_struct_name: ::cuke_runner::glue::step::StaticStepDef =
            ::cuke_runner::glue::step::StaticStepDef {
                name: stringify!(#user_handler_fn_name),
                keyword: #keyword,
                expression: #expression,
                step_fn: #generated_fn_name,
                step_fn_location_fn: #generated_location_fn_name,
            };
    }.into())
}

fn complete_step(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let function: syn::ItemFn = syn::parse2(input)
        .map_err(Diagnostic::from)
        .map_err(|diag| diag.help("`#[step]` can only be used on functions"))?;

    let full_attr = quote!(#[step(#args)]);
    let attrs = Attribute::parse_outer.parse2(full_attr)
        .map_err(Diagnostic::from)?;
    let attribute = match StepAttribute::from_attrs("step", &attrs) {
        Some(result) => result?,
        None => return Err(Span::call_site().error("internal error: bad attribute"))
    };

    codegen_step(parse_step(attribute, function)?)
}

fn incomplete_step(
    keyword: crate::glue::step::StepKeyword,
    args: TokenStream,
    input: TokenStream
) -> Result<TokenStream> {
    let keyword_str = keyword.to_string().to_lowercase();
    // FIXME(proc_macro): there should be a way to get this `Span`.
    let keyword_span = StringLit::new(format!("#[{}]", keyword), Span::call_site())
        .subspan(2..2 + keyword_str.len());
    let keyword_ident = syn::Ident::new(&keyword_str, keyword_span.into());

    let function: syn::ItemFn = syn::parse2(input)
        .map_err(Diagnostic::from)
        .map_err(|d| d.help(format!("#[{}] can only be used on functions", keyword_str)))?;

    let full_attr = quote!(#[#keyword_ident(#args)]);
    let attrs = Attribute::parse_outer.parse2(full_attr)
        .map_err(Diagnostic::from)?;
    let keyword_attribute = match KeywordStepAttribute::from_attrs(&keyword_str, &attrs) {
        Some(result) => result?,
        None => return Err(Span::call_site().error("internal error: bad attribute"))
    };

    let attribute = StepAttribute {
        keyword: SpanWrapped {
            full_span: keyword_span,
            span: keyword_span,
            value: StepKeyword(keyword),
        },
        expression: keyword_attribute.expression,
    };

    codegen_step(parse_step(attribute, function)?)
}

pub fn step_attribute<K: Into<Option<crate::glue::step::StepKeyword>>>(
    keyword: K,
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream
) -> TokenStream {
    let result = match keyword.into() {
        Some(keyword) => incomplete_step(keyword, args.into(), input.into()),
        None => complete_step(args.into(), input.into())
    };

    result.unwrap_or_else(|diag| diag.emit_as_item_tokens())
}
