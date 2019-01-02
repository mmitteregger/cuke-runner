extern crate failure;
#[macro_use]
extern crate failure_derive;

mod hook_type;
mod step_keyword;
mod scenario;
mod from_scenario;
mod code_location;
mod step_definition;
mod hook_definition;
mod handler;

pub use hook_type::*;
pub use step_keyword::*;
pub use scenario::*;
pub use from_scenario::*;
pub use code_location::*;
pub use step_definition::*;
pub use hook_definition::*;
pub use handler::*;

pub struct StaticGlueDefinitions {
    pub before_scenario_hooks: &'static [&'static StaticHookDefinition],
    pub before_step_hooks: &'static [&'static StaticHookDefinition],
    pub steps: &'static [&'static StaticStepDefinition],
    pub after_step_hooks: &'static [&'static StaticHookDefinition],
    pub after_scenario_hooks: &'static [&'static StaticHookDefinition],
}
