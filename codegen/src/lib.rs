#![crate_type = "dylib"]
#![feature(quote, concat_idents, plugin_registrar, rustc_private)]
#![feature(custom_attribute)]

extern crate cuke_runner;
extern crate syntax;
extern crate syntax_ext;
extern crate syntax_pos;
extern crate rustc_plugin;
extern crate rustc_target;
//extern crate symbol;
extern crate regex;
#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
mod utils;
mod decorators;
mod parser;
mod macros;

use std::env;
use rustc_plugin::Registry;
use syntax::ext::base::SyntaxExtension;
use syntax::symbol::Symbol;

const DEBUG_ENV_VAR: &'static str = "CUKE_CODEGEN_DEBUG";

const PARAM_PREFIX: &'static str = "cuke_param_";
const STEP_STRUCT_PREFIX: &'static str = "static_cuke_step_info_for_";
const STEP_FN_PREFIX: &'static str = "cuke_step_fn_";

const STEP_ATTR: &'static str = "cuke_step";
const STEP_INFO_ATTR: &'static str = "cuke_step_info";

macro_rules! register_macros {
    ($reg:expr, $($n:expr => $f:ident),+) => (
        $($reg.register_macro($n, macros::$f);)+
    )
}

macro_rules! register_decorators {
    ($registry:expr, $($name:expr => $func:ident),+) => (
        $($registry.register_syntax_extension(Symbol::intern($name),
                SyntaxExtension::MultiModifier(Box::new(decorators::$func)));
         )+
    )
}

/// Compiler hook for Rust to register plugins.
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    // Enable logging early if the DEBUG_ENV_VAR is set.
    if env::var(DEBUG_ENV_VAR).is_ok() {
        env_logger::Builder::from_default_env()
            .filter(None, log::LevelFilter::Debug)
            .init();
    }

    register_macros!(reg,
        "cuke_runner" => cuke_runner
    );

    register_decorators!(reg,
        "step" => step_decorator,
        "given" => given_decorator,
        "when" => when_decorator,
        "then" => then_decorator
    );
}
