use proc_macro::{Span, TokenStream};

use devise::{FromMeta, Result, Spanned, SpanWrapped, syn};
use proc_macro2::TokenStream as TokenStream2;

use {STEP_FN_PREFIX, STEP_STRUCT_PREFIX};
use attribute::GlueFnArg;
use glue_codegen::{Regex, StepKeyword};
use path_utils;
use proc_macro_ext::{Diagnostics, StringLit};
use syn_ext::{IdentExt, syn_to_diag};

use self::syn::{Attribute, parse::Parser};

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

fn step_data_expr(ident: &syn::Ident, ty: &syn::Type, step_argument_index: usize) -> TokenStream2 {
    let span = ident.span().unstable().join(ty.span()).unwrap().into();

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
    let user_handler_fn_span = &user_handler_fn.sig.ident.span().unstable();
    let user_handler_fn_path = path_utils::source_file_path(&user_handler_fn_span.source_file());
    let user_handler_fn_file_path_str = path_utils::path_to_str(&user_handler_fn_path);
    let user_handler_fn_line_number = user_handler_fn_span.start().line;
    let generated_fn_name = user_handler_fn_name.prepend(STEP_FN_PREFIX);
    let generated_struct_name = user_handler_fn_name.prepend(STEP_STRUCT_PREFIX);
    let parameter_names = step.arguments.iter().map(|argument| &argument.cuke_runner_ident);
    let keyword = step.attribute.keyword;
    let expression = step.attribute.expression;

    let mut step_argument_index = 0;
    let data_statements = step.arguments
        .iter()
        .map(|argument| {
            if argument.scenario_arg {
                super::scenario_data_expr(&argument.cuke_runner_ident, &argument.ty)
            } else {
                let step_data_expr = step_data_expr(&argument.cuke_runner_ident, &argument.ty, step_argument_index);
                step_argument_index += 1;
                step_data_expr
            }
        })
        .collect::<Vec<_>>();

    Ok(quote! {
        #[inline(never)] // to see the function in the stack trace in case of a panic
        #user_handler_fn

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
                location: ::cuke_runner::glue::location::StaticGlueCodeLocation {
                    file_path: #user_handler_fn_file_path_str,
                    line_number: #user_handler_fn_line_number,
                },
            };
    }.into())
}

fn complete_step(args: TokenStream2, input: TokenStream) -> Result<TokenStream> {
    let function: syn::ItemFn = syn::parse(input).map_err(syn_to_diag)
        .map_err(|diag| diag.help("`#[step]` can only be used on functions"))?;

    let full_attr = quote!(#[step(#args)]);
    let attrs = Attribute::parse_outer.parse2(full_attr).map_err(syn_to_diag)?;
    let attribute = match StepAttribute::from_attrs("step", &attrs) {
        Some(result) => result?,
        None => return Err(Span::call_site().error("internal error: bad attribute"))
    };

    codegen_step(parse_step(attribute, function)?)
}

fn incomplete_step(
    keyword: ::glue::step::StepKeyword,
    args: TokenStream2,
    input: TokenStream
) -> Result<TokenStream> {
    let keyword_str = keyword.to_string().to_lowercase();
    // FIXME(proc_macro): there should be a way to get this `Span`.
    let keyword_span = StringLit::new(format!("#[{}]", keyword), Span::call_site())
        .subspan(2..2 + keyword_str.len())
        .unwrap_or_else(Span::call_site);
    let keyword_ident = syn::Ident::new(&keyword_str, keyword_span.into());

    let function: syn::ItemFn = syn::parse(input).map_err(syn_to_diag)
        .map_err(|d| d.help(format!("#[{}] can only be used on functions", keyword_str)))?;

    let full_attr = quote!(#[#keyword_ident(#args)]);
    let attrs = Attribute::parse_outer.parse2(full_attr).map_err(syn_to_diag)?;
    let keyword_attribute = match KeywordStepAttribute::from_attrs(&keyword_str, &attrs) {
        Some(result) => result?,
        None => return Err(Span::call_site().error("internal error: bad attribute"))
    };

    let attribute = StepAttribute {
        keyword: SpanWrapped {
            full_span: keyword_span, span: keyword_span, value: StepKeyword(keyword)
        },
        expression: keyword_attribute.expression,
    };

    codegen_step(parse_step(attribute, function)?)
}

pub fn step_attribute<K: Into<Option<::glue::step::StepKeyword>>>(
    keyword: K,
    args: TokenStream,
    input: TokenStream
) -> TokenStream {
    let result = match keyword.into() {
        Some(keyword) => incomplete_step(keyword, args.into(), input),
        None => complete_step(args.into(), input)
    };

    result.unwrap_or_else(|diag| { diag.emit(); TokenStream::new() })
}
