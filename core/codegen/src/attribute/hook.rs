use devise::{FromMeta, Result, SpanWrapped, Diagnostic};
use devise::ext::SpanDiagnosticExt;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{Attribute, parse::Parser};

use crate::{
    AFTER_SCENARIO_HOOK_FN_LOCATION_FN_PREFIX,
    AFTER_SCENARIO_HOOK_FN_PREFIX,
    AFTER_SCENARIO_HOOK_STRUCT_PREFIX,
    AFTER_STEP_HOOK_FN_LOCATION_FN_PREFIX,
    AFTER_STEP_HOOK_FN_PREFIX,
    AFTER_STEP_HOOK_STRUCT_PREFIX,
    BEFORE_SCENARIO_HOOK_FN_LOCATION_FN_PREFIX,
    BEFORE_SCENARIO_HOOK_FN_PREFIX,
    BEFORE_SCENARIO_HOOK_STRUCT_PREFIX,
    BEFORE_STEP_HOOK_FN_LOCATION_FN_PREFIX,
    BEFORE_STEP_HOOK_FN_PREFIX,
    BEFORE_STEP_HOOK_STRUCT_PREFIX,
};
use crate::attribute::GlueFnArg;
use crate::glue_codegen::{HookType, TagExpression};
use crate::proc_macro_ext::{Diagnostics, StringLit};
use crate::syn_ext::IdentExt;

/// The raw, parsed `#[hook]` attribute.
#[derive(Debug, FromMeta)]
struct HookAttribute {
    #[meta(naked)]
    hook_type: SpanWrapped<HookType>,
    order: Option<isize>,
    tag_expression: Option<TagExpression>,
}

/// The raw, parsed `#[hook]` (e.g, `before_scenario`, `before_step`, ...) attribute.
#[derive(Debug, FromMeta)]
struct HookTypeHookAttribute {
    order: Option<isize>,
    tag_expression: Option<TagExpression>,
}

/// This structure represents the parsed `hook` attribute and associated items.
#[derive(Debug)]
struct Hook {
    /// The status associated with the code in the `#[hook(code)]` attribute.
    attribute: HookAttribute,
    /// The function that was decorated with the `step` attribute.
    function: syn::ItemFn,
    /// Parsed function arguments.
    arguments: Vec<GlueFnArg>,
}

fn parse_hook(attr: HookAttribute, mut function: syn::ItemFn) -> Result<Hook> {
    // Gather diagnostics as we proceed.
    let mut diags = Diagnostics::new();

    let arguments = super::parse_glue_fn_args(&mut diags, &mut function);

    diags.head_err_or(Hook { attribute: attr, function, arguments })
}

fn generate_location_fn_name(user_handler_fn_name: &Ident, hook_type: &HookType) -> Ident {
    use crate::glue::hook::HookType::*;

    let hook_fn_location_fn_prefix = match hook_type.0 {
        BeforeScenario => BEFORE_SCENARIO_HOOK_FN_LOCATION_FN_PREFIX,
        BeforeStep => BEFORE_STEP_HOOK_FN_LOCATION_FN_PREFIX,
        AfterStep => AFTER_STEP_HOOK_FN_LOCATION_FN_PREFIX,
        AfterScenario => AFTER_SCENARIO_HOOK_FN_LOCATION_FN_PREFIX,
    };

    user_handler_fn_name.prepend(hook_fn_location_fn_prefix)
}

fn generate_fn_name(user_handler_fn_name: &Ident, hook_type: &HookType) -> Ident {
    use crate::glue::hook::HookType::*;

    let hook_fn_prefix = match hook_type.0 {
        BeforeScenario => BEFORE_SCENARIO_HOOK_FN_PREFIX,
        BeforeStep => BEFORE_STEP_HOOK_FN_PREFIX,
        AfterStep => AFTER_STEP_HOOK_FN_PREFIX,
        AfterScenario => AFTER_SCENARIO_HOOK_FN_PREFIX,
    };

    user_handler_fn_name.prepend(hook_fn_prefix)
}

fn generate_struct_name(user_handler_fn_name: &Ident, hook_type: &HookType) -> Ident {
    use crate::glue::hook::HookType::*;

    let hook_struct_prefix = match hook_type.0 {
        BeforeScenario => BEFORE_SCENARIO_HOOK_STRUCT_PREFIX,
        BeforeStep => BEFORE_STEP_HOOK_STRUCT_PREFIX,
        AfterStep => AFTER_STEP_HOOK_STRUCT_PREFIX,
        AfterScenario => AFTER_SCENARIO_HOOK_STRUCT_PREFIX,
    };

    user_handler_fn_name.prepend(hook_struct_prefix)
}

fn codegen_hook(hook: Hook) -> Result<TokenStream> {
    // Gather everything we need.
    let (vis, user_handler_fn) = (&hook.function.vis, &hook.function);
    let user_handler_fn_name = &user_handler_fn.sig.ident;
    let user_handler_fn_span = user_handler_fn.sig.ident.span();
    let hook_type = hook.attribute.hook_type;
    let generated_location_fn_name = generate_location_fn_name(user_handler_fn_name, &hook_type.value);
    let generated_fn_name = generate_fn_name(user_handler_fn_name, &hook_type.value);
    let generated_struct_name = generate_struct_name(user_handler_fn_name, &hook_type.value);
    let parameter_names = hook.arguments.iter().map(|argument| &argument.cuke_runner_ident);
    let order = hook.attribute.order.unwrap_or(0);
    let tag_expression = hook.attribute.tag_expression
        .map(|t| t.0)
        .unwrap_or_else(String::new);

    let mut data_statements = Vec::with_capacity(hook.arguments.len());
    for argument in &hook.arguments {
        let data_statement = super::scenario_data_expr(&argument)?;
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

        /// Cuke runner code generated wrapping hook function.
        #vis fn #generated_fn_name(
            __scenario: &mut ::cuke_runner::glue::scenario::Scenario,
        ) -> ::std::result::Result<(), ::cuke_runner::glue::error::ExecutionError> {

            #(#data_statements)*

            // TODO: error handling...
            let result = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| #user_handler_fn_name(#(#parameter_names),*)));
            match result {
                Ok(user_handler_fn_result) => return Ok(()),
                Err(_err) => return Err(::cuke_runner::glue::error::ExecutionError::Panic(::cuke_runner::glue::error::PanicError::new())),
            };
        }

        /// Cuke runner code generated static hook info.
        #[allow(non_upper_case_globals)]
        #vis static #generated_struct_name: ::cuke_runner::glue::hook::StaticHookDef =
            ::cuke_runner::glue::hook::StaticHookDef {
                name: stringify!(#user_handler_fn_name),
                order: #order,
                tag_expression: #tag_expression,
                hook_fn: #generated_fn_name,
                hook_fn_location_fn: #generated_location_fn_name,
            };
    }.into())
}

fn complete_hook(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let function: syn::ItemFn = syn::parse2(input)
        .map_err(Diagnostic::from)
        .map_err(|diag| diag.help("`#[hook]` can only be used on functions"))?;

    let full_attr = quote!(#[hook(#args)]);
    let attrs = Attribute::parse_outer.parse2(full_attr).map_err(Diagnostic::from)?;
    let attribute = match HookAttribute::from_attrs("hook", &attrs) {
        Some(result) => result?,
        None => return Err(Span::call_site().error("internal error: bad attribute"))
    };

    codegen_hook(parse_hook(attribute, function)?)
}

fn incomplete_hook(
    hook_type: crate::glue::hook::HookType,
    args: TokenStream,
    input: TokenStream
) -> Result<TokenStream> {
    let hook_type_str = hook_type.to_string().to_lowercase();
    // FIXME(proc_macro): there should be a way to get this `Span`.
    let hook_type_span = StringLit::new(format!("#[{}]", hook_type), Span::call_site())
        .subspan(2..2 + hook_type_str.len());
    let hook_type_ident = syn::Ident::new(&hook_type_str, hook_type_span.into());

    let function: syn::ItemFn = syn::parse2(input).map_err(Diagnostic::from)
        .map_err(|d| d.help(format!("#[{}] can only be used on functions", hook_type_str)))?;

    let full_attr = quote!(#[#hook_type_ident(#args)]);
    let attrs = Attribute::parse_outer.parse2(full_attr)
        .map_err(Diagnostic::from)?;
    let hook_type_attribute = match HookTypeHookAttribute::from_attrs(&hook_type_str, &attrs) {
        Some(result) => result?,
        None => return Err(Span::call_site().error("internal error: bad attribute"))
    };

    let attribute = HookAttribute {
        hook_type: SpanWrapped {
            full_span: hook_type_span,
            span: hook_type_span,
            value: HookType(hook_type),
        },
        order: hook_type_attribute.order,
        tag_expression: hook_type_attribute.tag_expression,
    };

    codegen_hook(parse_hook(attribute, function)?)
}

pub fn hook_attribute<T: Into<Option<crate::glue::hook::HookType>>>(
    hook_type: T,
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream
) -> TokenStream {
    let result = match hook_type.into() {
        Some(hook_type) => incomplete_hook(hook_type, args.into(), input.into()),
        None => complete_hook(args.into(), input.into())
    };

    result.unwrap_or_else(|diag| diag.emit_as_item_tokens())
}
