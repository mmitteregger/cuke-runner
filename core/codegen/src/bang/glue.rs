use proc_macro2::TokenStream;
use devise::{Result, Spanned, Diagnostic};
use syn::{parse::Parser, Path, punctuated::Punctuated, token::Comma};
use quote::{quote, quote_spanned};

pub fn glue_macro(input: TokenStream) -> Result<TokenStream> {
    let current_file_path = super::get_current_file_path();
    let base_path = current_file_path.parent().unwrap();
    let canonicalized_base_path = match base_path.canonicalize() {
        Ok(canonicalized_path) => canonicalized_path,
        Err(_) => base_path.to_owned(),
    };
    let base_path_str = path_to_str(canonicalized_base_path.as_path());

    let paths = <Punctuated<Path, Comma>>::parse_terminated
        .parse2(input)
        .map_err(Diagnostic::from)?;

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
        Glue::from(
            (
                std::path::PathBuf::from(#base_path_str),
                &[
                    #(#static_glue_definitions,
                    )*
                ] as &[::cuke_runner::glue::StaticGlueDefinitions],
            )
        )
    };

    Ok(TokenStream::from(glue))
}

fn path_to_str(path: &std::path::Path) -> &str {
    match path.to_str() {
        Some(path_str) => path_str,
        None => {
            panic!("Path \"{}\" cannot be losslessly converted to an UTF-8 string \
            and is thus currently not supported", path.display());
        },
    }
}
