mod generate_glue;
mod glue;

use proc_macro::TokenStream;

pub fn generate_glue_macro(input: TokenStream) -> TokenStream {
    generate_glue::generate_glue_macro(input)
        .map_err(|diag| diag.emit())
        .unwrap_or_else(|_| quote!(()).into())
}

pub fn glue_macro(input: TokenStream) -> TokenStream {
    glue::glue_macro(input)
        .map_err(|diag| diag.emit())
        .unwrap_or_else(|_| quote!(()).into())
}
