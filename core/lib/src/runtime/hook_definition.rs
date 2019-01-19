use std::fmt;

use gherkin::pickle::PickleTag;

use api::CodeLocation;
use error::Result;
use glue::hook::{HookFn, StaticHookDef};
use glue::hook::TagPredicate;
use runtime::Scenario;

#[derive(Clone)]
pub struct HookDefinition {
    tag_predicate: TagPredicate,
    order: u32,
//    timeout: Option<Duration>,
    hook_fn: HookFn,
    location: CodeLocation,
}

impl fmt::Debug for HookDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        f.debug_struct("HookDefinition")
            .field("tag_predicate", &self.tag_predicate)
            .field("order", &self.order)
//            .field("timeout", &self.timeout)
            .field("hook_fn", &"<hook_fn>")
            .field("location", &self.location)
            .finish()
    }
}

impl From<&&StaticHookDef> for HookDefinition {
    fn from(static_hook_def: &&StaticHookDef) -> Self {
        let tag_predicate = TagPredicate::new(static_hook_def.tag_expression)
            .unwrap_or_else(|err| {
                panic!("tag expression \"{}\"is invalid: {}\n\
                        It should have been checked by codegen already, this is a bug!",
                    static_hook_def.tag_expression, err);
            });

        HookDefinition {
            tag_predicate,
            order: 0,
            hook_fn: static_hook_def.hook_fn,
            location: static_hook_def.location,
        }
    }
}

impl HookDefinition {
    pub fn get_location(&self) -> &CodeLocation {
        &self.location
    }

    pub fn execute(&self, scenario: &mut Scenario) -> Result<()> {
        let result = (self.hook_fn)(&mut scenario.glue_scenario);
        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(::error::Error::Execution(error)),
        }
    }

    pub fn matches(&self, tags: &[PickleTag]) -> bool {
        self.tag_predicate.test(tags)
    }

    pub fn get_order(&self) -> u32 {
        self.order
    }
}
