extern crate calculator;
#[macro_use]
extern crate cuke_runner;
#[macro_use]
extern crate cuke_runner_codegen;

// TODO: Can these mod inclusions be automatically managed by a macro
mod steps;

cuke_runner!();
