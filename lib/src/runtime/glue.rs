use std::collections::HashMap;

use codegen::StaticStepDefinition;
use runtime::{StepDefinition, HookDefinition};

#[derive(Debug)]
pub struct Glue {
    step_definitions_by_pattern: HashMap<String, StepDefinition>,
    before_scenario_hooks: Vec<HookDefinition>,
    before_step_hooks: Vec<HookDefinition>,
    after_step_hooks: Vec<HookDefinition>,
    after_scenario_hooks: Vec<HookDefinition>,
}

impl From<&'static [&'static StaticStepDefinition]> for Glue {
    fn from(step_definitions: &'static [&'static StaticStepDefinition]) -> Glue {
        Glue {
            step_definitions_by_pattern: HashMap::new(),
            before_scenario_hooks: Vec::new(),
            before_step_hooks: Vec::new(),
            after_step_hooks: Vec::new(),
            after_scenario_hooks: Vec::new(),
        }
    }
}

impl Glue {
    pub fn new() -> Glue {
        Glue {
            step_definitions_by_pattern: HashMap::new(),
            before_scenario_hooks: Vec::new(),
            before_step_hooks: Vec::new(),
            after_step_hooks: Vec::new(),
            after_scenario_hooks: Vec::new(),
        }
    }

    pub fn from_static_step_definitions(_static_step_definitions: &[&StaticStepDefinition]) -> Glue {
        Glue {
            step_definitions_by_pattern: HashMap::new(),
            before_scenario_hooks: Vec::new(),
            before_step_hooks: Vec::new(),
            after_step_hooks: Vec::new(),
            after_scenario_hooks: Vec::new(),
        }
    }

//    pub fn add_static_step_definition(&mut self, static_step_definition: StaticStepDefinition) {
//
//    }

    pub fn get_step_definitions_by_pattern(&self) -> &HashMap<String, StepDefinition> {
        &self.step_definitions_by_pattern
    }

    pub fn get_before_scenario_hooks(&self) -> &Vec<HookDefinition> {
        &self.before_scenario_hooks
    }

    pub fn get_before_step_hooks(&self) -> &Vec<HookDefinition> {
        &self.before_step_hooks
    }

    pub fn get_after_step_hooks(&self) -> &Vec<HookDefinition> {
        &self.after_step_hooks
    }

    pub fn get_after_scenario_hooks(&self) -> &Vec<HookDefinition> {
        &self.after_scenario_hooks
    }
}
