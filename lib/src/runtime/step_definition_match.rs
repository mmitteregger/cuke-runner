use std::fmt::Debug;
use std::any::Any;

use gherkin::pickle::PickleStep;

use api::FnDefLocation;
use runtime::{HookDefinition, Scenario, StepDefinition};
use runtime::step_expression::Argument;

pub trait StepDefinitionMatch: Debug + Send + Sync {
    fn run_step(&self, language: &str, scenario: &mut Scenario);

    fn dry_run_step(&self, language: &str, scenario: &mut Scenario);

    fn get_code_location(&self) -> &FnDefLocation;
}


#[derive(Debug)]
pub struct HookDefinitionMatch {
    pub hook_definition: HookDefinition,
}

impl StepDefinitionMatch for HookDefinitionMatch {
    fn run_step(&self, language: &str, scenario: &mut Scenario) {
        self.hook_definition.execute(scenario);
    }

    fn dry_run_step(&self, language: &str, scenario: &mut Scenario) {
        // Do nothing
    }

    fn get_code_location(&self) -> &FnDefLocation {
        self.hook_definition.get_location()
    }
}

#[derive(Debug)]
pub struct PickleStepDefinitionMatch {
    arguments: Vec<Box<Argument>>,
    step_definition: StepDefinition,
    feature_path: String,
    step: PickleStep,
}

impl PickleStepDefinitionMatch {
    pub fn new(arguments: Vec<Box<Argument>>, step_definition: StepDefinition, feature_path: String,
               step: PickleStep)
        -> PickleStepDefinitionMatch
    {
        PickleStepDefinitionMatch {
            arguments,
            step_definition,
            feature_path,
            step,
        }
    }
}

impl PickleStepDefinitionMatch {
    pub fn get_pattern(&self) -> &String {
        self.step_definition.get_pattern()
    }

    pub fn get_arguments(&self) -> &Vec<Box<Argument>> {
        &self.arguments
    }

    pub fn get_location(&self) -> &FnDefLocation {
        self.step_definition.get_location()
    }
}

impl StepDefinitionMatch for PickleStepDefinitionMatch {
    fn run_step(&self, language: &str, scenario: &mut Scenario) {
        unimplemented!();
    }

    fn dry_run_step(&self, language: &str, scenario: &mut Scenario) {
        // Do nothing
    }

    fn get_code_location(&self) -> &FnDefLocation {
        self.step_definition.get_location()
    }
}
