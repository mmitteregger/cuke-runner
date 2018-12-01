#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_span)]
#![feature(proc_macro_def_site)]
#![feature(crate_visibility_modifier)]
#![feature(rustc_private)]
#![recursion_limit="128"]

extern crate cuke_runner_glue as glue;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate regex;
extern crate syn;
#[macro_use]
extern crate log;
extern crate devise;
extern crate indexmap;
extern crate syntax_pos;

use proc_macro2::Span;
use proc_macro::TokenStream;
use std::env;
use syn::Path;

use glue::StepKeyword;

#[macro_use]
mod proc_macro_ext;
mod attribute;
mod step_definitions;
mod glue_codegen;
mod syn_ext;

crate static STEP_STRUCT_PREFIX: &str = "static_cuke_runner_step_info_for_";
crate static STEP_FN_PREFIX: &str = "cuke_runner_step_fn_";
crate static PARAM_PREFIX: &str = "__cuke_runner_param_";

macro_rules! emit {
    ($tokens:expr) => ({
        let tokens = $tokens;
        if ::std::env::var_os("CUKE_CODEGEN_DEBUG").is_some() {
            ::proc_macro::Span::call_site()
                .note("emitting cuke runner code generation debug output")
                .note(tokens.to_string())
                .emit()
        }

        tokens
    })
}

macro_rules! step_attribute {
    ($name:ident => $method:expr) => (
        #[proc_macro_attribute]
        pub fn $name(args: TokenStream, input: TokenStream) -> TokenStream {
            emit!(attribute::step::step_attribute($method, args, input))
        }
    )
}

step_attribute!(step => StepKeyword::Star);
step_attribute!(given => StepKeyword::Given);
step_attribute!(when => StepKeyword::When);
step_attribute!(then => StepKeyword::Then);


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
        pub static STEP_DEFINITIONS: &[&::cuke_runner::glue::StaticStepDefinition] = &[
            #(&#step_definition_path_tokens,
            )*
        ];
    };

    TokenStream::from(step_definition_tokens)
}
