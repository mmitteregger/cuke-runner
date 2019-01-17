/*!
Cucumber for Rust with a focus on ease-of-use.
*/
#![feature(nll)]
#![feature(arbitrary_self_types)]

#[allow(unused_imports)] #[macro_use] extern crate cuke_runner_codegen;
#[doc(hidden)] pub use cuke_runner_codegen::*;

pub extern crate cuke_runner_glue as glue;

extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate gherkin;
extern crate walkdir;
extern crate regex;
extern crate rayon;
extern crate termcolor;

pub use config::{Config, ExecutionMode};
pub use error::{Error, Result};
pub use runtime::Glue;

mod config;
mod error;
mod parser;
pub mod api;
mod runner;
mod runtime;

pub fn execute_cucumber_tests(glue: Glue, config: Config) {
    let exit_status = runtime::run(glue, config);

    if exit_status != 0 {
        panic!("Cucumber test(s) failed");
    }
}
