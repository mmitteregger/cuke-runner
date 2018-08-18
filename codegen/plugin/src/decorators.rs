use std::fmt::Display;

use ::{PARAM_PREFIX, STEP_STRUCT_PREFIX, STEP_FN_PREFIX};
use ::{STEP_ATTR, STEP_DEFINITION_ATTR};

use utils::*;
use parser::{StepParams};

use syntax::codemap::{Span, Spanned};
use syntax::tokenstream::TokenTree;
use syntax::ast::{Arg, Ident, Item, Stmt, Expr, MetaItem, Path, TyKind};
use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::ext::build::AstBuilder;
use syntax::parse::token;
use syntax::print::pprust;
use syntax::symbol::LocalInternedString;
use syntax::ptr::P;

use cuke_runner::data::StepKeyword;

fn keyword_to_path(ecx: &ExtCtxt, keyword: StepKeyword) -> Path {
    quote_enum!(ecx, keyword => ::cuke_runner::data::StepKeyword {
        Given, When, Then, Star;
    })
}

pub fn step_decorator(
    ecx: &mut ExtCtxt, sp: Span, meta_item: &MetaItem, annotated: Annotatable
) -> Vec<Annotatable> {
    generic_step_decorator(None, ecx, sp, meta_item, annotated)
}

macro_rules! step_keyword_decorator {
    ($name:ident, $keyword:ident) => (
        pub fn $name(
            ecx: &mut ExtCtxt, sp: Span, meta_item: &MetaItem, annotated: Annotatable
        ) -> Vec<Annotatable> {
            let i_sp = meta_item.span.shorten_to(stringify!($keyword).len());
            let step_keyword = Some(span(StepKeyword::$keyword, i_sp));
            generic_step_decorator(step_keyword, ecx, sp, meta_item, annotated)
        }
    )
}

step_keyword_decorator!(given_decorator, Given);
step_keyword_decorator!(when_decorator, When);
step_keyword_decorator!(then_decorator, Then);

fn generic_step_decorator(known_keyword: Option<Spanned<StepKeyword>>,
    ecx: &mut ExtCtxt,
    sp: Span,
    meta_item: &MetaItem,
    annotated: Annotatable
) -> Vec<Annotatable> {
    let mut output = Vec::new();

    // Parse the step and generate the code to create the param vars.
    let step = StepParams::from(ecx, sp, known_keyword, meta_item, &annotated);
    debug!("Step params: {:?}", step);

    let param_statements = step.generate_param_statements(ecx);
    let fn_arguments = step.generate_fn_arguments(ecx);

    // Generate and emit the wrapping function with the handler signature.
    let user_fn_name = step.annotated_fn.ident();
    let step_fn_name = user_fn_name.prepend(STEP_FN_PREFIX);
    emit_item(&mut output, quote_item!(ecx,
        // Allow the `unreachable_code` lint for those FromParam impls that have
        // an `Error` associated type of !.
        #[allow(unreachable_code)]
        pub fn $step_fn_name(__step_data: &::cuke_runner::data::StepData) -> ::cuke_runner::Result<()> {
            $param_statements
            // TODO: $user_fn_name should be able to return nothing () or a cuke_runner::Result
            $user_fn_name($fn_arguments);
            Ok(())
        }
    ).unwrap());

    // Generate and emit the static step info that uses the just generated
    // function as its handler. A proper Rocket step will be created from this.
    let struct_name = user_fn_name.prepend(STEP_STRUCT_PREFIX);
    let (name, text, keyword) = step.explode(ecx);
    let static_step_info_item =  quote_item!(ecx,
        /// Cuke Runner code generated static step information structure.
        #[allow(non_upper_case_globals)]
        pub static $struct_name: ::cuke_runner::codegen::StaticStepDefinition =
            ::cuke_runner::codegen::StaticStepDefinition {
                name: $name,
                keyword: $keyword,
                text: $text,
                handler: $step_fn_name,
            };
    ).expect("static step info");

    // Attach a `cuke_step_definition` attribute to the step info and emit it.
    let attr_name = Ident::from_str(STEP_DEFINITION_ATTR);
    let info_attr = quote_attr!(ecx, #[$attr_name]);
    attach_and_emit(&mut output, info_attr, Annotatable::Item(static_step_info_item));

    // Attach a `cuke_step` attribute to the user's function and emit it.
    let attr_name = Ident::from_str(STEP_ATTR);
    let step_attr = quote_attr!(ecx, #[$attr_name($struct_name)]);
    attach_and_emit(&mut output, step_attr, annotated);
//    emit_annotatable(&mut output, annotated);

    output
}

impl StepParams {
    fn missing_declared_err<T: Display>(&self, ecx: &ExtCtxt, arg: &Spanned<T>) {
        let (fn_span, fn_name) = (self.annotated_fn.span(), self.annotated_fn.ident());
        ecx.struct_span_err(arg.span, &format!("unused dynamic parameter: `{}`", arg.node))
            .span_note(fn_span, &format!("expected argument named `{}` in `{}` handler",
                arg.node, fn_name))
            .emit();
    }

    fn generate_param_statements(&self, ecx: &ExtCtxt) -> Vec<Stmt> {
        let mut fn_param_statements = vec![];

        // Generate the code for `from_step_data` parameters.
        let all = &self.annotated_fn.decl().inputs;
        for arg in all.iter() {
            let ident = arg.ident().unwrap().prepend(PARAM_PREFIX);
            let ty = strip_ty_lifetimes(arg.ty.clone());

            fn_param_statements.push(quote_stmt!(ecx,
                #[allow(non_snake_case, unreachable_patterns)]
                let $ident: $ty = match
                        ::cuke_runner::data::FromStepData::from_step_data(__step_data) {
                    Ok(step_data) => step_data,
                    Err(error) => {
                        return Err(error.into())
                    },
                };
            ).expect("undeclared param parsing statement"));
        }

        fn_param_statements
    }

    fn generate_fn_arguments(&self, ecx: &ExtCtxt) -> Vec<TokenTree> {
        let args = self.annotated_fn.decl().inputs.iter()
            .filter_map(|a| a.ident())
            .map(|ident| ident.prepend(PARAM_PREFIX))
            .collect::<Vec<Ident>>();

        sep_by_tok(ecx, &args, token::Comma)
    }

    fn explode(&self, ecx: &ExtCtxt) -> (LocalInternedString, &str, Path) {
        let name = self.annotated_fn.ident().name.as_str();
        let text = &self.text.node.as_str();
        let keyword = keyword_to_path(ecx, self.keyword.node);

        (name, text, keyword)
    }
}
