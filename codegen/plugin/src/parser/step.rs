use cuke_runner::data::StepKeyword;
use regex::Regex;
use std::str::FromStr;
use super::Function;
use super::regex::validate_regex;
use syntax::ast::*;
use syntax::codemap::{dummy_spanned, Span, Spanned};
use syntax::ext::base::{Annotatable, ExtCtxt};
use utils::{MetaItemExt, span};

/// This structure represents the parsed `step` attribute.
///
/// It contains all of the information supplied by the user and the span where
/// the user supplied the information. This structure can only be obtained by
/// calling the `StepParams::from` function and passing in the entire decorator
/// environment.
#[derive(Debug)]
pub struct StepParams {
    pub annotated_fn: Function,
    pub keyword: Spanned<StepKeyword>,
    pub text: Spanned<Regex>,
}

impl StepParams {
    /// Parses the step attribute from the given decorator context. If the
    /// parse is not successful, this function exits early with the appropriate
    /// error message to the user.
    pub fn from(
        ecx: &mut ExtCtxt,
        sp: Span,
        known_keyword: Option<Spanned<StepKeyword>>,
        meta_item: &MetaItem,
        annotated: &Annotatable
    ) -> StepParams {
        let function = Function::from(annotated).unwrap_or_else(|item_sp| {
            ecx.span_err(sp, "this attribute can only be used on functions...");
            ecx.span_fatal(item_sp, "...but was applied to the item below.");
        });

        let meta_items = meta_item.meta_item_list().unwrap_or_else(|| {
            ecx.struct_span_err(sp, "incorrect use of attribute")
                .help("attributes in cuke_runner must have the form: #[name(...)]")
                .emit();
            ecx.span_fatal(sp, "malformed attribute");
        });

        if meta_items.len() < 1 {
            ecx.span_fatal(sp, "attribute requires at least 1 parameter");
        }

        let (keyword, attr_params) = match known_keyword {
            Some(step_keyword) => (step_keyword, meta_items),
            None => (parse_keyword(ecx, &meta_items[0]), &meta_items[1..])
        };

        if attr_params.len() < 1 {
            ecx.struct_span_err(sp, "attribute requires at least a text")
                .help(r#"example: #[given("step text")] or #[given(text = "step text")]"#)
                .emit();
            ecx.span_fatal(sp, "malformed attribute");
        }

        let text = parse_text(ecx, &attr_params[0]);

        StepParams {
            keyword,
            text,
            annotated_fn: function,
        }
    }
}

fn parse_keyword(ecx: &ExtCtxt, meta_item: &NestedMetaItem) -> Spanned<StepKeyword> {
    let default_keyword = dummy_spanned(StepKeyword::Star);
    let valid_keywords = "valid keywords are: `Given`, `When`, `Then`, `*`";

    if let Some(word) = meta_item.word() {
        if let Ok(keyword) = StepKeyword::from_str(&word.name().as_str()) {
            return span(keyword, word.span());
        }

        let msg = format!("'{}' is not a valid step keyword", word.ident);
        ecx.struct_span_err(word.span, &msg).help(valid_keywords).emit();
        return default_keyword;
    }

    // Fallthrough. Emit a generic error message and return default keyword.
    let msg = "expected a valid step keyword identifier";
    ecx.struct_span_err(meta_item.span, msg).help(valid_keywords).emit();
    dummy_spanned(StepKeyword::Star)
}

fn parse_text(ecx: &ExtCtxt, meta_item: &NestedMetaItem) -> Spanned<Regex> {
    let sp = meta_item.span();
    if let Some((name, lit)) = meta_item.name_value() {
        if name != "text" {
            ecx.span_err(sp, "the first key, if any, must be 'text'");
        } else if let LitKind::Str(ref s, _) = lit.node {
            return validate_regex(ecx, &s.as_str(), lit.span);
        } else {
            ecx.span_err(lit.span, "`text` value must be a string")
        }
    } else if let Some(s) = meta_item.str_lit() {
        return validate_regex(ecx, &s.as_str(), sp);
    } else {
        ecx.struct_span_err(sp, r#"expected `text = string` or a regex string"#)
            .help(r#"you can specify the text directly as a string, \
                  e.g: "a simple step text", or as a key-value pair, \
                  e.g: text = "a simple step text" "#)
            .emit();
    }

    dummy_spanned(Regex::new("").unwrap())
}
