use proc_macro::Diagnostic;

use devise::{ext::TypeExt, Result, Spanned, syn};
use proc_macro2::{Span as Span2, TokenStream as TokenStream2};

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
        syn::Type::Reference(type_reference) => {
            from_scenario_expr(type_reference, span)
        }
        /*
        Example:
        Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: [
                    PathSegment {
                        ident: Ident {
                            ident: "Option",
                            span: #0 bytes(19180..19186)
                        },
                        arguments: AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Lt,
                            args: [
                                Type(Reference(TypeReference {
                                    and_token: And,
                                    lifetime: None,
                                    mutability: None,
                                    elem: Path(TypePath {
                                        qself: None,
                                        path: Path {
                                            leading_colon: None,
                                            segments: [
                                                PathSegment {
                                                    ident: Ident {
                                                        ident: "<CustomTypeName>",
                                                        span: #0 bytes(19188..19195)
                                                    },
                                                    arguments: None
                                                }
                                            ]
                                        }
                                    })
                                }))
                            ],
                            gt_token: Gt
                        })
                    }
                ]
            }
        })
        */
        syn::Type::Path(type_path) => {
            let first_path_segment = if type_path.path.segments.len() == 1 {
                &type_path.path.segments[0]
            } else {
                return Err(invalid_scenario_data_argument(argument));
            };

            let option_path_segment = if first_path_segment.ident == "Option" {
                first_path_segment
            } else {
                return Err(invalid_scenario_data_argument(argument));
            };

            let option_argument = match &option_path_segment.arguments {
                syn::PathArguments::AngleBracketed(arguments) => {
                    if arguments.args.len() == 1 {
                        &arguments.args[0]
                    } else {
                        return Err(invalid_scenario_data_argument(argument));
                    }
                }
                _ => return Err(invalid_scenario_data_argument(argument)),
            };

            let option_type = match option_argument {
                syn::GenericArgument::Type(option_type) => option_type,
                _ => return Err(invalid_scenario_data_argument(argument)),
            };

            let option_type_reference = match option_type {
                syn::Type::Reference(option_type_reference) => option_type_reference,
                _ => return Err(invalid_scenario_data_argument(argument)),
            };

            from_scenario_expr(option_type_reference, span)
        }
        _ => return Err(invalid_scenario_data_argument(argument)),
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

fn from_scenario_expr(type_reference: &syn::TypeReference, span: Span2) -> TokenStream2 {
    if type_reference.mutability.is_some() {
        quote_spanned! { span =>
            ::cuke_runner::glue::scenario::FromScenarioMut::from_scenario_mut(__scenario)
        }
    } else {
        quote_spanned! { span =>
            ::cuke_runner::glue::scenario::FromScenario::from_scenario(__scenario)
        }
    }
}

fn invalid_scenario_data_argument(argument: &GlueFnArg) -> Diagnostic {
    argument.user_ident
        .span()
        .unstable()
        .error("unsupported scenario data argument, must be one of:
        * shared reference: &T
        * mutable reference: &mut T
        * optional shared reference: Option<&T>
        * optional mutable reference: Option<&mut T>")
}
