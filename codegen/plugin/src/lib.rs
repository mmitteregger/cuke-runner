#![crate_type = "dylib"]
#![feature(quote, concat_idents, plugin_registrar, rustc_private)]
#![feature(custom_attribute)]

extern crate cuke_runner;
extern crate syntax;
extern crate syntax_ext;
extern crate syntax_pos;
extern crate proc_macro;
extern crate rustc_plugin;
extern crate rustc_target;
//extern crate symbol;
extern crate regex;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate syn;
extern crate proc_macro2;

#[macro_use]
mod utils;
mod decorators;
mod parser;

use std::env;
use rustc_plugin::Registry;
use syntax::ext::base::SyntaxExtension;
use syntax::symbol::Symbol;

const DEBUG_ENV_VAR: &'static str = "CUKE_CODEGEN_DEBUG";

const PARAM_PREFIX: &'static str = "cuke_param_";
const STEP_STRUCT_PREFIX: &'static str = "static_step_definition_for_";
const STEP_FN_PREFIX: &'static str = "cuke_step_fn_";

const STEP_ATTR: &'static str = "cuke_step";
const STEP_DEFINITION_ATTR: &'static str = "cuke_step_definition";

macro_rules! register_decorators {
    ($registry:expr, $($name:expr => $func:ident),+ $(,)*) => (
        $($registry.register_syntax_extension(Symbol::intern($name),
                SyntaxExtension::MultiModifier(Box::new(decorators::$func)));
         )+
    )
}

//macro_rules! register_macros {
//    ($registry:expr, $($n:expr => $f:ident),+ $(,)*) => (
//        $($registry.register_macro($n, macros::$f);)+
//    )
//}

/// Compiler hook for Rust to register plugins.
#[plugin_registrar]
pub fn plugin_registrar(registry: &mut Registry) {
    // Enable logging early if the DEBUG_ENV_VAR is set.
    if let Ok(debug_env_var) = env::var(DEBUG_ENV_VAR) {
        if debug_env_var.as_str() == "true" {
            env_logger::Builder::from_default_env()
                .filter(None, log::LevelFilter::Debug)
                .init();
        }
    }

    // Keep these in sync with STEP_FN_ATTR_NAMES
    register_decorators!(registry,
        "step" => step_decorator,
        "given" => given_decorator,
        "when" => when_decorator,
        "then" => then_decorator,
    );

//    register_macros!(registry,
//        "cuke_runner" => cuke_runner,
//    );
}

