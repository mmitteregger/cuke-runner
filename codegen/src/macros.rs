use std::env;
use std::path::PathBuf;

use STEP_STRUCT_PREFIX;
use utils::{sep_by_tok, span, ParserExt, IdentExt};

use rustc_target::spec::abi::Abi;
use syntax::codemap::Span;
use syntax::tokenstream::TokenTree;
use syntax::ast::{self, Path, Expr, Unsafety, FunctionRetTy, FnDecl, BareFnTy, Block,
    BlockCheckMode, Visibility, VisibilityKind, Ident, ItemKind, Item, Stmt, Constness, Generics};
use syntax::ext::base::{DummyResult, ExtCtxt, MacResult, MacEager};
use syntax::parse::token::Token;
use syntax::ptr::P;
use syntax::util::small_vector::SmallVector;
//use symbol::{Symbol, keywords};

//#[macro_export]
//macro_rules! cuke_runner {
//    () => {
//        #[test]
//        fn cukes() {
//            let mut tests_base_path = $crate::macros::__project_dir(); // -> /../../<crate>
//            let current_file_path = file!(); // -> tests/cukes.rs
//            tests_base_path.push(current_file_path); // -> /../../<crate>/tests/cukes.rs
//            tests_base_path.pop(); // -> /../../<crate>/tests
//
//            $crate::run_cukes(&tests_base_path);
//        }
//    };
//}

//#[macro_export]
//#[doc(hidden)]
//macro_rules! __cuke_runner_include_steps {
//() => {
//        mod steps {
//            mod rpn_calculator {
//                include!("steps/rpn_calculator.rs");
//            }
//        }
//    };
//}

#[rustfmt_skip]
pub fn cuke_runner(ecx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
//    MacEager::items(SmallVector::one(P(Item {
//        ident: Ident::from_str("fn"),
//        attrs: Vec::new(),
//        id: ast::DUMMY_NODE_ID,
//        node: ItemKind::Fn(
//            P(FnDecl {
//                inputs: Vec::new(),
//                output: FunctionRetTy::Default(sp),
//                variadic: false,
//            }),
//            Unsafety::Normal,
//            span(Constness::NotConst, sp),
//            Abi::Rust,
//            Generics::default(),
//            P(Block {
//                stmts: Vec::<Stmt>::new(),
//                id: ast::DUMMY_NODE_ID,
//                rules: BlockCheckMode::Default,
//                span: sp,
//                recovered: false,
//            }),
//        ),
//        vis: span(VisibilityKind::Inherited, sp),
//        span: sp,
//        tokens: None,
//    })))
//    ::syntax::ast::Item {
//        ident: ::syntax::ast::Ident::new("cuke_runner"),
//        attrs: Vec::new(),
//        id: NodeId,
//        node: Item_,
//        vis: Visibility,
//        span: Span,
//    }

    MacEager::items(SmallVector::one(quote_item!(ecx,
        #[test]
        fn cukes() {
            let mut tests_base_path = ::cuke_runner::__cuke_runner_project_dir(); // -> /../../<crate>
            tests_base_path.push("tests"); // -> /../../<crate>/tests

            ::cuke_runner::run_cukes(&tests_base_path);
        }
    ).unwrap()))
}

