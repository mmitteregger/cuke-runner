use syntax::codemap::{dummy_spanned, Span, Spanned};
use syntax::ext::base::ExtCtxt;
use utils;

use regex::Regex;

pub fn validate_regex(ecx: &ExtCtxt, string: &str, sp: Span) -> Spanned<Regex> {
    match Regex::new(string) {
        Ok(regex) => utils::span(regex, sp),
        Err(error) => {
            ecx.struct_span_err(sp, "regex is not valid")
                .note(&format!("step regex \"{}\" is invalid: {}", string, error))
                .emit();
            dummy_spanned(Regex::new("").unwrap())
        }
    }
}
