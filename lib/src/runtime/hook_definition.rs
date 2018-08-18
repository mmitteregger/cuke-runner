use std::time::Duration;

use gherkin::pickle::PickleTag;

use api::SourceCodeLocation;
use error::Result;
use codegen::HookHandler;
use runtime::Scenario;
use super::TagPredicate;

#[derive(Debug, Clone)]
pub struct HookDefinition {
    tag_predicate: TagPredicate,
    order: u32,
    timeout: Option<Duration>,
    handler: HookHandler,
    location: SourceCodeLocation,
}

impl HookDefinition {
    pub fn get_location(&self) -> &SourceCodeLocation {
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
