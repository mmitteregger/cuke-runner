/*!
Cucumber for Rust with a focus on ease-of-use.
*/
#![feature(nll)]

extern crate downcast_rs;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate gherkin;
#[macro_use]
extern crate lazy_static;
extern crate state;
extern crate walkdir;
extern crate regex;
extern crate rayon;

use std::path::Path;

use gherkin::ast::*;

pub use config::{Config, ExecutionMode};
pub use data::State;
pub use error::{Error, Result};
pub use runtime::Glue;

mod config;
mod error;
#[doc(hidden)]
pub mod codegen;
pub mod data;
mod parser;
pub mod api;
mod runner;
mod runtime;

pub fn execute_cucumber_tests(glue: Glue, config: Config) {
    let exit_status = runtime::run(glue, config);

    if exit_status != 0 {
        eprintln!("Uh oh, looks like some cukes have rotten");
        ::std::process::exit(-1);
    }
}
