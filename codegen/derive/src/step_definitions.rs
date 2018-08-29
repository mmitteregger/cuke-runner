use std::fs;
use std::ops::Add;
use std::path::PathBuf;
use syn::{self, Attribute, Item, Visibility};

const STEP_STRUCT_PREFIX: &'static str = "static_step_definition_for_";
const STEP_FN_ATTR_NAMES: &[&str] = &["step", "given", "when", "then"];

pub fn parse(root_path: &mut PathBuf) -> Vec<String> {
    let mut static_step_definitions = Vec::new();

    let mut module_paths = Vec::new();

    add_from_file(&mut static_step_definitions, &mut module_paths, root_path);

    static_step_definitions
}

fn add_from_file(static_step_definitions: &mut Vec<String>,
    module_paths: &mut Vec<String>, fs_path: &mut PathBuf) {

    debug!("Searching for step definitions in: {}", &fs_path.display());

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
                    debug!("Skipping private function: {}", step_fn_path);
                    continue;
                }

                if get_step_attribute(item_fn.attrs).is_none() {
                    debug!("Skipping function without step attribute: {}", step_fn_path);
                    continue;
                }

                let function_path = String::new()
                    .add("::")
                    .add(&current_path)
                    .add("::")
                    .add(STEP_STRUCT_PREFIX)
                    .add(&function_name);

                static_step_definitions.push(function_path);
            },
            Item::Mod(item_mod) => {
                let module_name = item_mod.ident.to_string();

//                if !is_visible(item_mod.vis) {
//                    debug!("Skipping private module: {}", module_path);
//                    continue;
//                }
                if !item_mod.attrs.is_empty() {
                    panic!("attributes on step module declarations are not supported yet")
                }

                if let Some(content) = item_mod.content {
                    let _brace = content.0;
                    let module_items = content.1;

                    module_paths.push(module_name);
                    add_from_items(static_step_definitions, module_paths, fs_path, module_items);
                    module_paths.pop();
                } else if let Some(_semi) = item_mod.semi {
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
