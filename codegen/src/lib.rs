#![crate_type = "dylib"]
#![feature(quote, concat_idents, plugin_registrar, rustc_private)]
#![feature(custom_attribute)]

extern crate syntax;
extern crate syntax_ext;
extern crate syntax_pos;
extern crate rustc_plugin;
extern crate regex;
extern crate cuke_runner;

mod decorators;
mod parser;
mod utils;

const STEP_FN_PREFIX: &'static str = "cuke_runner_step_fn_";

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
    register_decorators!(reg,
        "step" => step_decorator,
        "given" => given_decorator,
        "when" => when_decorator,
        "then" => then_decorator
    );
}
