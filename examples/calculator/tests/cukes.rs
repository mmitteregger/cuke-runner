#![feature(plugin, decl_macro)]
#![plugin(cuke_runner_codegen)]

extern crate calculator;
extern crate cuke_runner;

mod steps;

cuke_runner!();
