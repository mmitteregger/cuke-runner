mod step_fn_paths;

use ::{STEP_FN_ATTR_NAMES, STEP_STRUCT_PREFIX};
use cuke_runner;
use rustc_target::spec::abi::Abi;
use std::env;
use std::fs;
use std::path::PathBuf;
use syntax::ast::*;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacEager, MacResult};
use syntax::print::pprust::item_to_string;
use syntax::tokenstream::TokenTree;
use syntax::util::small_vector::SmallVector;
use syntax_pos::symbol::Symbol;
use utils::span;

pub fn cuke_runner(ecx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    // Path to the directory where the file which invoked this macro exists
    let root_path: &PathBuf = &ecx.root_path.canonicalize().unwrap();
    let root_path_string = root_path.to_str()
        .expect("cannot create UTF-8 string out of current path");
    let root_path_lit = LitKind::Str(Symbol::intern(root_path_string), StrStyle::Raw(2));
    let root_path_token = span(root_path_lit, sp);

    let step_fn_paths = step_fn_paths::find_step_fn_paths(&root_path);
    debug!("Found step function paths: {:?}", step_fn_paths);

    let item = quote_item!(ecx,
        #[test]
        fn cukes() {
            let root_path = ::std::path::PathBuf::from($root_path_token);
            debug_assert!(root_path.exists(), "expected root path \"{}\" to exist",
                    root_path.display());

            ::cuke_runner::run_cukes(root_path);
        }
    ).unwrap();

    debug!("Emitting item:\n{}", item_to_string(&item));
    MacEager::items(SmallVector::one(item))
}
