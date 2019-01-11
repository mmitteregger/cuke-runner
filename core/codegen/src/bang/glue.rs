use proc_macro::TokenStream;
use syn_ext::syn_to_diag;
use devise::{syn, Spanned, Result};

use self::syn::{Path, punctuated::Punctuated, parse::Parser, token::Comma};

crate fn glue_macro(input: TokenStream) -> Result<TokenStream> {
    let paths = <Punctuated<Path, Comma>>::parse_terminated
        .parse(input)
        .map_err(syn_to_diag)?;

    let static_glue_definitions = paths.into_iter()
        .map(|path| quote_spanned! {path.span().into()=>
            ::cuke_runner::glue::StaticGlueDefinitions {
                before_scenario_hooks: #path::BEFORE_SCENARIO_HOOK_DEFINITIONS,
                before_step_hooks: #path::BEFORE_STEP_HOOK_DEFINITIONS,
                steps: #path::STEP_DEFINITIONS,
                after_step_hooks: #path::AFTER_STEP_HOOK_DEFINITIONS,
                after_scenario_hooks: #path::AFTER_SCENARIO_HOOK_DEFINITIONS,
            }
        });

    let glue = quote! {
        Glue::from(&[
            #(#static_glue_definitions,
            )*
        ] as &[::cuke_runner::glue::StaticGlueDefinitions])
    };

    Ok(TokenStream::from(glue))
}
