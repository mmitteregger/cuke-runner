mod static_step_definition;

use ::{STEP_FN_ATTR_NAMES, STEP_STRUCT_PREFIX};
use rustc_target::spec::abi::Abi;
use std::env;
use std::path::PathBuf;
use std::collections::HashMap;
use std::iter::FromIterator;

use syntax::ast::*;
use syntax::parse::token::Token;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacEager, MacResult};
use syntax::print::pprust::item_to_string;
use syntax::tokenstream::{TokenTree, TokenStream};
use syntax::util::small_vector::SmallVector;
use syntax_pos::symbol::Symbol;
use proc_macro2;
use utils::{span, sep_by_tok};

//use cuke_runner::codegen::Glue;
//
//pub fn cuke_runner(ecx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
//    let call_site_span = ecx.call_site();
//
//    // Path to the directory where the file which invoked this macro exists
//    let root_path: &PathBuf = &ecx.root_path.canonicalize().unwrap();
//    let root_path_string = root_path.to_str()
//        .expect("cannot create UTF-8 string out of current path");
//    let root_path_lit = LitKind::Str(Symbol::intern(root_path_string), StrStyle::Raw(0));
//    let root_path_token = span(root_path_lit, sp);
//
//    let step_definitions = static_step_definition::search(&root_path);
//    debug!("Found static step definitions: {:?}", step_definitions);
//
//    let definitions_exprs = step_definitions.into_iter()
//        .map(|step_definition| {
//            Path::from_ident(Ident::from_str(&step_definition))
////            let name = step_definition.name;
////            let text = span(LitKind::Str(Symbol::intern(&step_definition.text), StrStyle::Cooked), call_site_span);
////            let handler = span(Path::from_ident(Ident::new(Symbol::intern(&step_definition.handler), call_site_span)), call_site_span);
////
////            quote_expr!(ecx, ::cuke_runner::codegen::StepDefinition {
////                expression: ::cuke_runner::codegen::StepExpression::from_regex($text),
////                parameter_infos: Vec::new(),
////                handler: $handler,
////                location: ::cuke_runner::api::SourceCodeLocation {
////                    file_path: String::new(),
////                    line_number: 0,
////                },
////            })
//        })
//        .collect::<Vec<_>>();
//    let definitions = sep_by_tok(ecx, &definitions_exprs, Token::Comma);
//
//    let item = quote_item!(ecx,
//        #[test]
//        fn cukes() {
//            let root_path = ::std::path::PathBuf::from($root_path_token);
//            debug_assert!(root_path.exists(), "expected root path \"{}\" to exist",
//                    root_path.display());
//
//            let glue = ::cuke_runner::codegen::Glue::from_static_step_definitions(&[$definitions]);
//
//            ::cuke_runner::run_cukes(root_path, glue);
//        }
//    ).unwrap();
//
////    use syntax::ext::quote::rt::ToTokens;
////    let tokens = item.to_tokens(ecx);
////
////    let token_streams = tokens.into_iter()
////        .map(|token| token.joint())
////        .collect::<Vec<TokenStream>>();
////    let token_stream = TokenStream::concat(token_streams);
////
////    let f = foo(token_stream, call_site_span);
////    let ft = f.as_tree().0;
////    let i = quote_item!(ecx, $ft).unwrap();
////
//////    let a = into_call_site_span()
////
////    debug!("Emitting item:\n{}", item_to_string(&i));
//////    ::syntax::ext::quote::expand_quote_item(ecx, call_site_span, &[item.into_inner().tokens.unwrap().as_tree().0])
////    MacEager::items(SmallVector::one(i))
//
//    debug!("Emitting item:\n{}", item_to_string(&item));
//    MacEager::items(SmallVector::one(item))
//}

//fn into_call_site_span(stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
//    let call_site_span = proc_macro2::Span::call_site();
//    let iter = stream.into_iter().map(|mut tree| {
//        match tree {
//            proc_macro2::TokenTree::Group(g) => {
//                proc_macro2::TokenTree::Group(proc_macro2::Group::new(g.delimiter(), into_call_site_span(g.stream())))
//            }
//            _ => {
//                tree.set_span(call_site_span);
//                tree
//            }
//        }
//    });
//    proc_macro2::TokenStream::from_iter(iter)
//}

fn foo(stream: TokenStream, call_site_span: Span) -> TokenStream {
    stream.map(|mut tree| {
        match tree {
            TokenTree::Token(span, token) => TokenTree::Token(call_site_span, token),
            TokenTree::Delimited(span, delimited) => TokenTree::Delimited(call_site_span, delimited),
        }
    })
}
