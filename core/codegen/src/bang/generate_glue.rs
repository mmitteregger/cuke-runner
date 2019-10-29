use std::fs;
use std::ops::Add;
use std::path::PathBuf;
use proc_macro2::Span;
use proc_macro::TokenStream;
use syn::{self, Path, Attribute, Item, Visibility};
use devise::Result;
use {
    BEFORE_SCENARIO_HOOK_STRUCT_PREFIX,
    BEFORE_STEP_HOOK_STRUCT_PREFIX,
    STEP_STRUCT_PREFIX,
    AFTER_STEP_HOOK_STRUCT_PREFIX,
    AFTER_SCENARIO_HOOK_STRUCT_PREFIX,
};

crate fn generate_glue_macro(_input: TokenStream) -> Result<TokenStream> {
    let current_file_path = super::get_current_file_path();

    let before_scenario_hook_definition_path_tokens: Vec<Path> = parse_paths(&current_file_path,
        &["before_scenario"], BEFORE_SCENARIO_HOOK_STRUCT_PREFIX);
    let before_step_hook_definition_path_tokens: Vec<Path> = parse_paths(&current_file_path,
        &["before_step"], BEFORE_STEP_HOOK_STRUCT_PREFIX);
    let step_definition_path_tokens = parse_paths(&current_file_path,
        &["given", "when", "then"], STEP_STRUCT_PREFIX);
    let after_step_hook_definition_path_tokens: Vec<Path> = parse_paths(&current_file_path,
        &["after_step"], AFTER_STEP_HOOK_STRUCT_PREFIX);
    let after_scenario_hook_definition_path_tokens: Vec<Path> = parse_paths(&current_file_path,
        &["after_scenario"], AFTER_SCENARIO_HOOK_STRUCT_PREFIX);

    let call_site_span = Span::call_site();
    let static_glue_definition_tokens = quote_spanned! {call_site_span=>
        pub static BEFORE_SCENARIO_HOOK_DEFINITIONS: &[&::cuke_runner::glue::hook::StaticHookDef] = &[
            #(&#before_scenario_hook_definition_path_tokens,
            )*
        ];
        pub static BEFORE_STEP_HOOK_DEFINITIONS: &[&::cuke_runner::glue::hook::StaticHookDef] = &[
            #(&#before_step_hook_definition_path_tokens,
            )*
        ];
        pub static STEP_DEFINITIONS: &[&::cuke_runner::glue::step::StaticStepDef] = &[
            #(&#step_definition_path_tokens,
            )*
        ];
        pub static AFTER_STEP_HOOK_DEFINITIONS: &[&::cuke_runner::glue::hook::StaticHookDef] = &[
            #(&#after_step_hook_definition_path_tokens,
            )*
        ];
        pub static AFTER_SCENARIO_HOOK_DEFINITIONS: &[&::cuke_runner::glue::hook::StaticHookDef] = &[
            #(&#after_scenario_hook_definition_path_tokens,
            )*
        ];
    };

    Ok(TokenStream::from(static_glue_definition_tokens))
}

fn parse_paths(root_path: &PathBuf, fn_attribute_names: &[&str], struct_prefix: &str) -> Vec<Path> {
    let mut static_glue_definition_paths = Vec::new();

    let mut module_paths = Vec::new();

    add_from_file(&mut static_glue_definition_paths, &mut module_paths, &mut root_path.clone(),
        fn_attribute_names, struct_prefix);

    static_glue_definition_paths
}

fn add_from_file(static_glue_definition_paths: &mut Vec<Path>,
    module_paths: &mut Vec<String>, fs_path: &mut PathBuf,
    fn_attribute_names: &[&str], struct_prefix: &str) {

    debug!("Searching for glue definitions in: {}", &fs_path.display());

    let src = match fs::read_to_string(&fs_path) {
        Ok(src) => src,
        Err(err) => panic!("could not read glue source file \"{}\": {}", fs_path.display(), err),
    };
    let syntax = match syn::parse_file(&src) {
        Ok(src) => src,
        Err(err) => panic!("unable to parse glue source file \"{}\": {}", fs_path.display(), err),
    };

    fs_path.pop();

    add_from_items(static_glue_definition_paths, module_paths, fs_path, syntax.items,
        fn_attribute_names, struct_prefix);
}

fn add_from_items(static_glue_definition_paths: &mut Vec<Path>,
    module_paths: &mut Vec<String>, fs_path: &mut PathBuf, items: Vec<Item>,
    fn_attribute_names: &[&str], struct_prefix: &str) {

    for item in items {
        match item {
            Item::Fn(item_fn) => {
                let current_path = module_paths.join("::");
                let function_name = item_fn.sig.ident.to_string();
                let glue_fn_path = format!("{}::{}", current_path, &function_name);

                if !is_visible(item_fn.vis) {
                    debug!("Skipping private function: {}", glue_fn_path);
                    continue;
                }

                if get_attribute(item_fn.attrs, fn_attribute_names).is_none() {
                    debug!("Skipping function without glue attribute: {}", glue_fn_path);
                    continue;
                }

                let function_path = String::new()
                    .add("::")
                    .add(&current_path)
                    .add("::")
                    .add(struct_prefix)
                    .add(&function_name);

                let path = syn::parse_str::<Path>(&function_path)
                    .expect("parse glue definition path");

                static_glue_definition_paths.push(path);
            },
            Item::Mod(item_mod) => {
                let module_name = item_mod.ident.to_string();

//                if !is_visible(item_mod.vis) {
//                    debug!("Skipping private module: {}", module_path);
//                    continue;
//                }

                for module_attribute in item_mod.attrs {
                    if module_attribute.path.is_ident("path") {
                        panic!("path attribute on glue module declarations are not supported yet");
                    }
                }

                if let Some(content) = item_mod.content {
                    let _brace = content.0;
                    let module_items = content.1;

                    module_paths.push(module_name);
                    add_from_items(static_glue_definition_paths, module_paths, fs_path, module_items,
                        fn_attribute_names, struct_prefix);
                    module_paths.pop();
                } else if let Some(_semi) = item_mod.semi {
                    fs_path.push(format!("{}.rs", &module_name));
                    if fs_path.exists() {
                        module_paths.push(module_name);
                        add_from_file(static_glue_definition_paths, module_paths, fs_path,
                            fn_attribute_names, struct_prefix);
                        module_paths.pop();
                        continue;
                    }
                    fs_path.pop();

                    fs_path.push(&module_name);
                    fs_path.push("mod.rs");
                    if fs_path.exists() {
                        module_paths.push(module_name);
                        add_from_file(static_glue_definition_paths, module_paths, fs_path,
                            fn_attribute_names, struct_prefix);
                        module_paths.pop();
                        continue;
                    }

                    panic!("could not find module: {}", fs_path.display());
                } else {
                    panic!("expected either glue module declaration with content or reference");
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
            panic!("restricted visibility for glues not supported yet")
        },
        Visibility::Inherited => false,
    }
}

fn get_attribute(attrs: Vec<Attribute>, fn_attribute_names: &[&str]) -> Option<Attribute> {
    for attr in attrs {
        let is_matching_attr = {
            let first_attr_path_segment = attr.path.segments.first()
                .expect("failed to find first attribute path segment");
            let attr_name = first_attr_path_segment.ident.to_string();
            fn_attribute_names.contains(&attr_name.as_ref())
        };

        if is_matching_attr {
            return Some(attr);
        }
    }

    None
}
