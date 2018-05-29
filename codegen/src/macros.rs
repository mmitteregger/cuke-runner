use ::{STEP_FN_ATTR_NAMES, STEP_STRUCT_PREFIX};
use cuke_runner;
use rustc_target::spec::abi::Abi;
use std::env;
use std::fs;
use std::path::{self, PathBuf};
use syn::{self, Attribute, Item, Visibility};
use syntax::ast::*;
use syntax::codemap::FilePathMapping;
use syntax::codemap::Span;
use syntax::ext::base::{DummyResult, ExtCtxt, MacEager, MacResult};
use syntax::parse::{new_parser_from_source_str, ParseSess};
use syntax::parse::token::Token;
use syntax::print::pprust::item_to_string;
use syntax::ptr::P;
use syntax::tokenstream::TokenTree;
use syntax::util::small_vector::SmallVector;
use syntax_pos::FileName;
use syntax_pos::symbol::Symbol;
use utils::{IdentExt, ParserExt, sep_by_tok, span};
use walkdir::{DirEntry, WalkDir};

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

pub fn cuke_runner(ecx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    // LET THERE BE MAGIC!
    // Path to the directory where the file which invoked this macro exists
    let root_path: &PathBuf = &ecx.root_path.canonicalize().unwrap();
    let root_path_string = root_path.to_str()
        .expect("cannot create UTF-8 string out of current path");
    let root_path_lit = LitKind::Str(Symbol::intern(root_path_string), StrStyle::Raw(2));
    let root_path_token = span(root_path_lit, sp);

    let step_fn_paths = find_step_fn_paths(&root_path);
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

fn find_step_fn_paths(root_path: &path::Path) -> Vec<String> {
    let mut step_fn_paths = Vec::new();

    let mut steps_module_path = root_path.join("steps");
    steps_module_path.push("mod.rs");

    let mut module_paths: Vec<String> = Vec::new();
    module_paths.push("steps".to_owned());

    add_step_fn_paths_from_file(&mut step_fn_paths, &mut module_paths, &mut steps_module_path);

//    let walk_dir = WalkDir::new(steps_dir)
//        .follow_links(true);
//
//    for entry in walk_dir {
//        let entry = entry.unwrap();
//        let path = entry.path();
//
//        if path.is_file() {
//            let step_module_path = path;
//
//            if step_module_path.ends_with("mod.rs") {
//                continue;
//            }

    step_fn_paths
}

fn add_step_fn_paths_from_file(step_fn_paths: &mut Vec<String>,
    module_paths: &mut Vec<String>, fs_path: &mut PathBuf) {

    // Deeper down the rabbit hole we go...
    debug!("Searching for step definitions in: {}", &fs_path.display());

    let src = fs::read_to_string(&fs_path).unwrap();
    let syntax = syn::parse_file(&src)
        .expect("unable to parse step source file");

    fs_path.pop();

    add_step_fn_paths_from_items(step_fn_paths, module_paths, fs_path, syntax.items);
}

fn add_step_fn_paths_from_items(step_fn_paths: &mut Vec<String>,
    module_paths: &mut Vec<String>, fs_path: &mut PathBuf, items: Vec<Item>) {

    for item in items {
        match item {
            Item::Fn(item_fn) => {
                let current_path = module_paths.join("::");
                let function_name = item_fn.ident.to_string();
                let step_fn_path = current_path + "::" + &function_name;

                if !is_visible(item_fn.vis) {
                    debug!("Skipping private function: {}", step_fn_path);
                    continue;
                }
                if !contains_step_attribute(item_fn.attrs) {
                    debug!("Skipping function without step attribute: {}", step_fn_path);
                    continue;
                }

                // TODO: Check for step attribute
                step_fn_paths.push(step_fn_path);
            },
            Item::Mod(item_mod) => {
                let current_path = module_paths.join("::");
                let module_name = item_mod.ident.to_string();
                let module_path = current_path + "::" + &module_name;

                if !is_visible(item_mod.vis) {
                    debug!("Skipping private module: {}", module_path);
                    continue;
                }
                if !item_mod.attrs.is_empty() {
                    panic!("attributes on step module declarations are not supported yet")
                }

                if let Some(content) = item_mod.content {
                    let _brace = content.0;
                    let module_items = content.1;

                    module_paths.push(module_name);
                    add_step_fn_paths_from_items(step_fn_paths, module_paths, fs_path,
                        module_items);
                    module_paths.pop();
                } else if let Some(semi) = item_mod.semi {
                    fs_path.push(format!("{}.rs", &module_name));
                    if fs_path.exists() {
                        module_paths.push(module_name);
                        add_step_fn_paths_from_file(step_fn_paths, module_paths, fs_path);
                        module_paths.pop();
                        continue;
                    }
                    fs_path.pop();

                    fs_path.push(&module_name);
                    fs_path.push("mod.rs");
                    if fs_path.exists() {
                        module_paths.push(module_name);
                        add_step_fn_paths_from_file(step_fn_paths, module_paths, fs_path);
                        module_paths.pop();
                        continue;
                    }

                    panic!("could not find module: {}", fs_path.display());
                } else {
                    panic!("expected either step module declaration with content or reference");
                }
            },
            _ => {},
        }
    }
}

fn is_visible(visibility: Visibility) -> bool {
    match visibility {
        Visibility::Public(_) => true,
        Visibility::Crate(_) => true,
        Visibility::Restricted(_) => {
            panic!("restricted visibility for steps not supported yet")
        },
        Visibility::Inherited => false,
    }
}

fn contains_step_attribute(attrs: Vec<Attribute>) -> bool {
    for attr in attrs {
        // our attributes only have one path segment
        let first_attr_path_segment = attr.path.segments.first().unwrap();
        let attr_name = first_attr_path_segment.value().ident.to_string();

        if STEP_FN_ATTR_NAMES.contains(&attr_name.as_ref()) {
            return true;
        }
    }

    false
}

//fn get_current_path(paths_stack: &Vec<String>) {
//    let current_path = String::new();
//
//
//    .last().unwrap().clone()
//}
