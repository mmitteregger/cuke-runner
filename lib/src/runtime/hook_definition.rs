use std::time::Duration;

use gherkin::pickle::PickleTag;

use api::FnDefLocation;
use error::Result;
use codegen::HookHandler;
use runtime::TagPredicate;
use runtime::Scenario;

#[derive(Debug, Clone)]
pub struct HookDefinition {
    tag_predicate: TagPredicate,
    order: u32,
    timeout: Option<Duration>,
    handler: HookHandler,
    location: FnDefLocation,
}

impl HookDefinition {
    pub fn get_location(&self) -> &FnDefLocation {
        &self.location
    }

    pub fn execute(&self, scenario: &mut Scenario) -> Result<()> {
        // TODO: Timeout and scenario parameter
        (self.handler)()
    }

    pub fn matches(&self, tags: &Vec<PickleTag>) -> bool {
        self.tag_predicate.apply_tags(tags)
    }

    pub fn get_order(&self) -> u32 {
        self.order
    }
}
