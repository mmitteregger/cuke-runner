use quote::ToTokens;
use proc_macro2::TokenStream as TokenStream2;
use devise::{FromMeta, MetaItem, Result};
use glue;

use proc_macro_ext::StringLit;

#[derive(Debug)]
crate struct HookType(crate glue::hook::HookType);

#[derive(Debug)]
crate struct StepKeyword(crate glue::step::StepKeyword);

#[derive(Debug)]
crate struct Regex(crate regex::Regex);

#[derive(Debug)]
crate struct TagExpression(crate String);

#[derive(Clone, Debug)]
crate struct Optional<T>(crate Option<T>);

impl FromMeta for StringLit {
    fn from_meta(meta: MetaItem) -> Result<Self> {
        Ok(StringLit::new(String::from_meta(meta)?, meta.value_span()))
    }
}

const VALID_HOOK_TYPES_STR: &str = "`BeforeScenario`, `BeforeStep`, `AfterStep`, `AfterScenario`";

const VALID_HOOK_TYPES: &[glue::hook::HookType] = &[
    glue::hook::HookType::BeforeScenario,
    glue::hook::HookType::BeforeStep,
    glue::hook::HookType::AfterStep,
    glue::hook::HookType::AfterScenario,
];

impl FromMeta for HookType {
    fn from_meta(meta: MetaItem) -> Result<Self> {
        let span = meta.value_span();
        let help_text = format!("hook type must be one of: {}", VALID_HOOK_TYPES_STR);

        if let MetaItem::Ident(ident) = meta {
            let hook_type = ident.to_string().parse()
                .map_err(|_| span.error("invalid hook type").help(&*help_text))?;

            if !VALID_HOOK_TYPES.contains(&hook_type) {
                return Err(span.error("invalid hook type for hook handlers").help(&*help_text));
            }

            return Ok(HookType(hook_type));
        }

        Err(span.error(format!("expected identifier, found {}", meta.description()))
                .help(&*help_text))
    }
}

impl ToTokens for HookType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        use glue::hook::HookType::*;

        let keyword_tokens = match self.0 {
            BeforeScenario => quote!(::cuke_runner::glue::HookType::BeforeScenario),
            BeforeStep => quote!(::cuke_runner::glue::HookType::BeforeStep),
            AfterStep => quote!(::cuke_runner::glue::HookType::AfterStep),
            AfterScenario => quote!(::cuke_runner::glue::HookType::AfterScenario),
        };

        tokens.extend(keyword_tokens);
    }
}

const VALID_STEPS_STR: &str = "`Given`, `When`, `Then`";

const VALID_STEPS: &[glue::step::StepKeyword] = &[
    glue::step::StepKeyword::Given,
    glue::step::StepKeyword::When,
    glue::step::StepKeyword::Then,
    glue::step::StepKeyword::Star,
];

impl FromMeta for StepKeyword {
    fn from_meta(meta: MetaItem) -> Result<Self> {
        let span = meta.value_span();
        let help_text = format!("keyword must be one of: {}", VALID_STEPS_STR);

        if let MetaItem::Ident(ident) = meta {
            let keyword = ident.to_string().parse()
                .map_err(|_| span.error("invalid keyword").help(&*help_text))?;

            if !VALID_STEPS.contains(&keyword) {
                return Err(span.error("invalid keyword for step handlers").help(&*help_text));
            }

            return Ok(StepKeyword(keyword));
        }

        Err(span.error(format!("expected identifier, found {}", meta.description()))
                .help(&*help_text))
    }
}

impl ToTokens for StepKeyword {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        use glue::step::StepKeyword::*;

        let keyword_tokens = match self.0 {
            Star => quote!(::cuke_runner::glue::step::StepKeyword::Star),
            Given => quote!(::cuke_runner::glue::step::StepKeyword::Given),
            When => quote!(::cuke_runner::glue::step::StepKeyword::When),
            Then => quote!(::cuke_runner::glue::step::StepKeyword::Then),
        };

        tokens.extend(keyword_tokens);
    }
}

impl FromMeta for Regex {
    fn from_meta(meta: MetaItem) -> Result<Self> {
        let string = StringLit::from_meta(meta)?;
        let span = string.subspan(1..=string.len())
            .unwrap_or_else(|| string.1.span());

        let result = regex::Regex::new(&string);
        match result {
            Ok(regex) => Ok(Regex(regex)),
            Err(err) => Err(span.error(format!("step expression \"{}\" is not a valid regex: {}", &*string, err))),
        }
    }
}

impl ToTokens for Regex {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let string = self.0.as_str();
        tokens.extend(quote!(#string));
    }
}

impl FromMeta for TagExpression {
    fn from_meta(meta: MetaItem) -> Result<Self> {
        let string = StringLit::from_meta(meta)?;
        let span = string.subspan(1..=string.len())
            .unwrap_or_else(|| string.1.span());

        let result = glue::hook::tag_predicate::parser::parse(string.as_ref());
        match result {
            Ok(_expression) => Ok(TagExpression(string.to_owned())),
            Err(err) => Err(span.error(format!("tag expression \"{}\" is invalid: {}", &*string, err))),
        }
    }
}

impl ToTokens for TagExpression {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let string = self.0.as_str();
        tokens.extend(quote!(#string));
    }
}

impl<T: ToTokens> ToTokens for Optional<T> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let opt_tokens = match self.0 {
            Some(ref val) => quote!(Some(#val)),
            None => quote!(None)
        };

        tokens.extend(opt_tokens);
    }
}
