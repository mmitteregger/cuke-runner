extern crate gherkin;
extern crate regex;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate indexmap;
extern crate backtrace;
#[macro_use]
extern crate lazy_static;

pub mod error;
pub mod scenario;
pub mod step;
pub mod hook;
pub mod panic;

use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct CodeLocation {
    pub file_path: &'static str,
    pub line_number: usize,
}

impl fmt::Display for CodeLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}:{}", self.file_path, self.line_number)
    }
}

pub struct StaticGlueDefinitions {
    pub before_scenario_hooks: &'static [&'static hook::StaticHookDef],
    pub before_step_hooks: &'static [&'static hook::StaticHookDef],
    pub steps: &'static [&'static step::StaticStepDef],
    pub after_step_hooks: &'static [&'static hook::StaticHookDef],
    pub after_scenario_hooks: &'static [&'static hook::StaticHookDef],
}
