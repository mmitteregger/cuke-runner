/*!
Cucumber for Rust with a focus on ease-of-use.
*/

#![warn(rust_2018_idioms)]

pub use gherkin;
#[doc(hidden)] pub use cuke_runner_codegen::*;
pub use cuke_runner_glue as glue;

pub use crate::config::{Config, ExecutionMode};
pub use crate::error::{Error, Result};
pub use crate::runtime::Glue;
pub use crate::runtime::event_listener;

mod config;
mod error;
pub mod api;
mod runner;
pub(crate) mod runtime;

pub fn execute_cucumber_tests(glue: Glue, config: Config<'_>) {
    let exit_status = runtime::run(glue, config);

    if exit_status != 0 {
        panic!("Cucumber test(s) failed");
    }
}
