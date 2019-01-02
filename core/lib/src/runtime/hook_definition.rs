use std::time::Duration;

use gherkin::pickle::PickleTag;

use api::CodeLocation;
use error::Result;
use glue::HookFn;
use runtime::Scenario;
use super::TagPredicate;

#[derive(Debug, Clone)]
pub struct HookDefinition {
    tag_predicate: TagPredicate,
    order: u32,
    timeout: Option<Duration>,
    hook_fn: HookFn,
    location: CodeLocation,
}

impl HookDefinition {
    pub fn get_location(&self) -> &CodeLocation {
        &self.location
    }

    pub fn execute(&self, scenario: &mut Scenario) -> Result<()> {
        // TODO: Timeout and scenario parameter
        let result = (self.hook_fn)();
        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(::error::Error::Execution(error)),
        }
    }

    pub fn matches(&self, tags: &Vec<PickleTag>) -> bool {
        self.tag_predicate.apply_tags(tags)
    }

    pub fn get_order(&self) -> u32 {
        self.order
    }
}
