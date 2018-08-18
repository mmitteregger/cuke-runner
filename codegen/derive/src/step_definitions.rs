use cuke_runner::codegen::StaticStepDefinition;
use cuke_runner::data::StepKeyword;
use proc_macro2::{TokenStream, TokenTree};
use std::fs;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use syn::{self, Attribute, Item, Visibility};

const STEP_STRUCT_PREFIX: &'static str = "static_step_definition_for_";
const STEP_FN_PREFIX: &'static str = "cuke_step_fn_";
const STEP_FN_ATTR_NAMES: &[&str] = &["step", "given", "when", "then"];

pub fn parse(mut root_path: &mut PathBuf) -> Vec<String> {
    let mut static_step_definitions = Vec::new();

//    let mut steps_module_path = root_path.join("steps");
//    steps_module_path.push("mod.rs");
//
    let mut module_paths = Vec::new();
//    module_paths.push("steps".to_owned());

//    add_from_file(&mut static_step_definitions, &mut module_paths, &mut steps_module_path);
    add_from_file(&mut static_step_definitions, &mut module_paths, root_path);

    static_step_definitions
}

fn add_from_file(static_step_definitions: &mut Vec<String>,
    module_paths: &mut Vec<String>, fs_path: &mut PathBuf) {

    // Deeper down the rabbit hole we go...
    println!("Searching for step definitions in: {}", &fs_path.display());

    let src = fs::read_to_string(&fs_path).unwrap();
    let syntax = syn::parse_file(&src)
        .expect("unable to parse step source file");

    fs_path.pop();

    add_from_items(static_step_definitions, module_paths, fs_path, syntax.items);
}

fn add_from_items(static_step_definitions: &mut Vec<String>,
    module_paths: &mut Vec<String>, fs_path: &mut PathBuf, items: Vec<Item>) {

    for item in items {
        match item {
            Item::Fn(item_fn) => {
                let current_path = module_paths.join("::");
                let function_name = item_fn.ident.to_string();
                let step_fn_path = format!("{}::{}", current_path, &function_name);

                if !is_visible(item_fn.vis) {
                    println!("Skipping private function: {}", step_fn_path);
                    continue;
                }

                let step_attr = match get_step_attribute(item_fn.attrs) {
                    Some(attr) => attr,
                    None => {
                        println!("Skipping function without step attribute: {}", step_fn_path);
                        continue;
                    },
                };

//                let attr_name = {
//                    let first_attr_path_segment = step_attr.path.segments.first().unwrap();
//                    first_attr_path_segment.value().ident.to_string()
//                };
//
//                let text = parse_text(step_attr);

                let function_path = String::new()
                    .add("::")
                    .add(&current_path)
                    .add("::")
                    .add(STEP_STRUCT_PREFIX)
                    .add(&function_name);

//                let static_step_definition = StaticStepDefinition {
//                    name: function_name,
//                    keyword: StepKeyword::from_str(&attr_name).unwrap(),
//                    text: text,
//                    handler: function_path,
//                };
                static_step_definitions.push(function_path);
            },
            Item::Mod(item_mod) => {
                let current_path = module_paths.join("::");
                let module_name = item_mod.ident.to_string();
                let module_path = current_path + "::" + &module_name;

//                if !is_visible(item_mod.vis) {
//                    println!("Skipping private module: {}", module_path);
//                    continue;
//                }
                if !item_mod.attrs.is_empty() {
                    panic!("attributes on step module declarations are not supported yet")
                }

                if let Some(content) = item_mod.content {
                    let _brace = content.0;
                    let module_items = content.1;

                    module_paths.push(module_name);
                    add_from_items(static_step_definitions, module_paths, fs_path,
                        module_items);
                    module_paths.pop();
                } else if let Some(semi) = item_mod.semi {
                    fs_path.push(format!("{}.rs", &module_name));
                    if fs_path.exists() {
                        module_paths.push(module_name);
                        add_from_file(static_step_definitions, module_paths, fs_path);
                        module_paths.pop();
                        continue;
                    }
                    fs_path.pop();

                    fs_path.push(&module_name);
                    fs_path.push("mod.rs");
                    if fs_path.exists() {
                        module_paths.push(module_name);
                        add_from_file(static_step_definitions, module_paths, fs_path);
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

fn get_step_attribute(attrs: Vec<Attribute>) -> Option<Attribute> {
    for attr in attrs {
        let is_step_attr = {
            let first_attr_path_segment = attr.path.segments.first().unwrap();
            let attr_name = first_attr_path_segment.value().ident.to_string();
            STEP_FN_ATTR_NAMES.contains(&attr_name.as_ref())
        };

        if is_step_attr {
            return Some(attr);
        }
    }

    None
}

fn parse_text(attr: Attribute) -> String {
    for token_tree in attr.tts {
        match token_tree {
            TokenTree::Group(group) => {
                for attr_token in group.stream() {
                    match attr_token {
                        TokenTree::Literal(lit) => {
                            let mut literal = lit.to_string();
                            literal.pop();
                            literal.remove(0);
                            return literal.replace("\\\\", "\\");
                        },
                        TokenTree::Group(g) => unimplemented!("parse_text from attr_token group: {}", g),
                        _ => panic!("Cannot parse text from attr_token: {}", attr_token),
                    }
                }

                panic!("Cannot parse text from attribute group: {}", group);
            }
            _ => panic!("Cannot parse text from attribute: {}", token_tree),
        }
    }

    panic!("Encountered attribute without any tokens!");
}
