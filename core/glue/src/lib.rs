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
#[doc(hidden)]
pub mod location;
#[doc(hidden)]
pub mod filter;
pub mod step;
pub mod hook;
pub mod panic;

#[doc(hidden)]
pub struct StaticGlueDefinitions {
    pub before_scenario_hooks: &'static [&'static hook::StaticHookDef],
    pub before_step_hooks: &'static [&'static hook::StaticHookDef],
    pub steps: &'static [&'static step::StaticStepDef],
    pub after_step_hooks: &'static [&'static hook::StaticHookDef],
    pub after_scenario_hooks: &'static [&'static hook::StaticHookDef],
}
