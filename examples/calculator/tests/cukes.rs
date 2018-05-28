#![feature(plugin, decl_macro)]
#![plugin(cuke_runner_codegen)]

extern crate calculator;
extern crate cuke_runner;

// TODO: Can these mod inclusions be automatically managed by a macro
mod steps;

cuke_runner!();
