#![recursion_limit="128"]

extern crate cuke_runner;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate regex;
extern crate syn;
#[macro_use]
extern crate log;

use proc_macro2::Span;
use proc_macro::TokenStream;
use std::env;
use syn::Path;

mod step_definitions;


#[proc_macro]
pub fn step_definitions(_input: TokenStream) -> TokenStream {
    let create_root_path = env::current_dir()
        .expect("current directory for crate root path");
    let crate_relative_path = env::args()
        .find(|arg| arg.ends_with(".rs"))
        .expect("could not find compiling rust file in current argument list");
    let mut current_file_path = create_root_path.join(crate_relative_path);

    debug!("current_file_path: {}", current_file_path.display());
    let step_definition_paths = step_definitions::parse(&mut current_file_path);
    debug!("step_definition_paths: {:?}", step_definition_paths);

    let step_definition_path_tokens = step_definition_paths.into_iter()
        .map(|step_definition_path| {
            syn::parse_str::<Path>(&step_definition_path).expect("parse step definition paths")
        })
        .collect::<Vec<_>>();

    let call_site_span = Span::call_site();
    let step_definition_tokens = quote_spanned! {call_site_span=>
        pub static STEP_DEFINITIONS: &[&::cuke_runner::codegen::StaticStepDefinition] = &[
            #(&#step_definition_path_tokens,
            )*
        ];
    };

    TokenStream::from(step_definition_tokens)
}
