use std::path::{Path, PathBuf};
use std::fs;

use syn::{self, Attribute, Item, Visibility};

use STEP_FN_ATTR_NAMES;

pub fn find_step_fn_paths(root_path: &Path) -> Vec<String> {
    let mut step_fn_paths = Vec::new();

    let mut steps_module_path = root_path.join("steps");
    steps_module_path.push("mod.rs");

    let mut module_paths = Vec::new();
    module_paths.push("steps".to_owned());

    add_step_fn_paths_from_file(&mut step_fn_paths, &mut module_paths, &mut steps_module_path);

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
