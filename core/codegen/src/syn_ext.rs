//! Extensions to `syn` types.

use devise::syn;
use quote::ToTokens;

pub trait IdentExt {
    fn prepend(&self, string: &str) -> syn::Ident;
}

impl IdentExt for syn::Ident {
    fn prepend(&self, string: &str) -> syn::Ident {
        syn::Ident::new(&format!("{}{}", string, self), self.span())
    }
}

pub trait PathExt {
    fn to_string(&self) -> String;
}

impl PathExt for syn::Path {
    fn to_string(&self) -> String {
        self.to_token_stream().to_string()
    }
}

pub trait ReturnTypeExt {
    fn ty(&self) -> Option<&syn::Type>;
}

impl ReturnTypeExt for syn::ReturnType {
    fn ty(&self) -> Option<&syn::Type> {
        match self {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => Some(ty),
        }
    }
}
