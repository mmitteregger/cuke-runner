use devise::{ext::TypeExt, Result, Spanned, syn};
use proc_macro2::TokenStream as TokenStream2;

use PARAM_PREFIX;
use proc_macro_ext::Diagnostics;
use syn_ext::IdentExt;

pub mod hook;
pub mod step;

/// Hook or step function argument.
#[derive(Debug)]
struct GlueFnArg {
    /// Whether the argument input should be taken from the scenario data or from the step data.
    /// This is determined by `#[scenario]` attribute presence.
    scenario_arg: bool,
    /// Argument name that the user wrote.
    user_ident: syn::Ident,
    /// Argument name that will be used by the code generation.
    cuke_runner_ident: syn::Ident,
    /// Argument type.
    ty: syn::Type,
}

fn parse_glue_fn_args(diags: &mut Diagnostics, function: &mut syn::ItemFn) -> Vec<GlueFnArg> {
    let inputs = &mut function.sig.inputs;
    let mut arguments = Vec::with_capacity(inputs.len());

    for arg in inputs.iter_mut() {
        let help = "all handler arguments must be of the form: `ident: Type`";
        let span = arg.span();

        match arg {
            syn::FnArg::Typed(arg) => {
                match *arg.pat {
                    syn::Pat::Ident(ref pat) => {
                        let mut scenario_arg = false;
                        arg.attrs.retain(|attr| {
                            if attr.path.is_ident("scenario") {
                                scenario_arg = true;
                                false
                            } else {
                                true
                            }
                        });
                        let user_ident = &pat.ident;
                        let ty = arg.ty.with_stripped_lifetimes();
                        let cuke_runner_ident = user_ident.prepend(PARAM_PREFIX);

                        arguments.push(GlueFnArg {
                            scenario_arg,
                            user_ident: user_ident.clone(),
                            cuke_runner_ident,
                            ty,
                        });
                    }
                    syn::Pat::Wild(_) => {
                        diags.push(span.error("handler arguments cannot be ignored").help(help));
                        continue;
                    }
                    _ => {
                        diags.push(span.error("invalid use of pattern").help(help));
                        continue;
                    }
                }
            }
            // Other cases shouldn't happen since we parsed an `ItemFn`.
            _ => {
                diags.push(span.error("invalid handler argument").help(help));
                continue;
            }
        }
    }

    arguments
}

fn scenario_data_expr(argument: &GlueFnArg) -> Result<TokenStream2> {
    let ty = &argument.ty;
    let ident = &argument.cuke_runner_ident;
    let span = ident.span().unstable().join(ty.span()).unwrap().into();
    let from_scenario = match ty {
        syn::Type::Reference(type_ref) => {
            if type_ref.mutability.is_some() {
                quote_spanned! { span =>
                    ::cuke_runner::glue::scenario::FromScenarioMut::from_scenario_mut(__scenario)
                }
            } else {
                quote_spanned! { span =>
                    ::cuke_runner::glue::scenario::FromScenario::from_scenario(__scenario)
                }
            }
        },
        _ => {
            let diagnostic = argument.user_ident
                .span()
                .unstable()
                .error("scenario data argument must be a shared or mutable reference");
            return Err(diagnostic);
        },
    };

    Ok(quote_spanned! { span =>
        #[allow(non_snake_case, unreachable_patterns)]
        let #ident: #ty = match #from_scenario {
            Ok(scenario_data) => scenario_data,
            Err(error) => {
                return Err(::cuke_runner::glue::error::ExecutionError::from(error))
            },
        };
    })
}
