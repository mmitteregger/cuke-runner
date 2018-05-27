use utils::*;
use parser::{Param, StepParams};

use syntax::codemap::{Span, Spanned, dummy_spanned};
use syntax::tokenstream::TokenTree;
use syntax::ast::{Arg, Ident, Item, Stmt, Expr, MetaItem, Path};
use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::ext::build::AstBuilder;
use syntax::parse::token;
use syntax::symbol::LocalInternedString;
use syntax::ptr::P;

use cuke_runner::StepKeyword;

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

fn generic_step_decorator(known_step_keyword: Option<Spanned<Method>>,
    ecx: &mut ExtCtxt,
    sp: Span,
    meta_item: &MetaItem,
    annotated: Annotatable
) -> Vec<Annotatable> {
    let mut output = Vec::new();

    // Parse the step and generate the code to create the param vars.
    let step = StepParams::from(ecx, sp, known_method, meta_item, &annotated);

    let param_statements = step.generate_param_statements(ecx);
    let query_statement = step.generate_query_statement(ecx);
    let data_statement = step.generate_data_statement(ecx);
    let fn_arguments = step.generate_fn_arguments(ecx);
    let uri_macro = step.generate_uri_macro(ecx);

    // Generate and emit the wrapping function with the Rocket handler signature.
    let user_fn_name = step.annotated_fn.ident();
    let route_fn_name = user_fn_name.prepend(ROUTE_FN_PREFIX);
    emit_item(&mut output, quote_item!(ecx,
        // Allow the `unreachable_code` lint for those FromParam impls that have
        // an `Error` associated type of !.
        #[allow(unreachable_code)]
        fn $route_fn_name<'_b>(__req: &'_b ::rocket::Request,  __data: ::rocket::Data)
                -> ::rocket::handler::Outcome<'_b> {
             $param_statements
             $query_statement
             $data_statement
             let responder = $user_fn_name($fn_arguments);
            ::rocket::handler::Outcome::from(__req, responder)
        }
    ).unwrap());

    // Generate and emit the static step info that uses the just generated
    // function as its handler. A proper Rocket step will be created from this.
    let struct_name = user_fn_name.prepend(ROUTE_STRUCT_PREFIX);
    let (name, path, method, media_type, rank) = step.explode(ecx);
    let static_route_info_item =  quote_item!(ecx,
        /// Rocket code generated static step information structure.
        #[allow(non_upper_case_globals)]
        pub static $struct_name: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                name: $name,
                method: $method,
                path: $path,
                handler: $route_fn_name,
                format: $media_type,
                rank: $rank,
            };
    ).expect("static step info");

    // Attach a `rocket_route_info` attribute to the step info and emit it.
    let attr_name = Ident::from_str(ROUTE_INFO_ATTR);
    let info_attr = quote_attr!(ecx, #[$attr_name]);
    attach_and_emit(&mut output, info_attr, Annotatable::Item(static_route_info_item));

    // Attach a `rocket_route` attribute to the user's function and emit it.
    let attr_name = Ident::from_str(ROUTE_ATTR);
    let route_attr = quote_attr!(ecx, #[$attr_name($struct_name)]);
    attach_and_emit(&mut output, route_attr, annotated);

    // Emit the per-step URI macro.
    emit_item(&mut output, uri_macro);

    output
}
