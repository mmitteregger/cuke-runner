use std::env;
use std::path::PathBuf;

use proc_macro2::TokenStream;
use quote::quote;

mod generate_glue;
mod glue;

pub fn generate_glue_macro(input: proc_macro::TokenStream) -> TokenStream {
    generate_glue::generate_glue_macro(input.into())
        .map_err(|diag| diag.emit_as_item_tokens())
        .unwrap_or_else(|_| quote!(()).into())
}

pub fn glue_macro(input: proc_macro::TokenStream) -> TokenStream {
    glue::glue_macro(input.into())
        .map_err(|diag| diag.emit_as_expr_tokens())
        .unwrap_or_else(|_| quote!(()).into())
}

fn get_current_file_path() -> PathBuf {
    let crate_root_path = env::current_dir()
        .expect("current directory for crate root path");
    let crate_relative_path = env::args()
        .find(|arg| arg.ends_with(".rs"))
        .expect("could not find compiling rust file in current argument list");
    let current_file_path = crate_root_path.join(crate_relative_path);

    log::debug!("current_file_path: {}", current_file_path.display());
    current_file_path
}
