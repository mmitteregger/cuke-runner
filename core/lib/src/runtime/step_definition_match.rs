use std::fmt::Debug;
use std::sync::Arc;

use gherkin::pickle::PickleStep;

use error::{Result, Error};
use api::CodeLocation;
use runtime::{Argument, HookDefinition, StepDefinition};
use runtime::Scenario;

pub trait StepDefinitionMatch: Debug + Send + Sync {
    fn get_step(&self) -> &PickleStep;

    fn run_step(&self, scenario: &mut Scenario) -> Result<()>;

    fn dry_run_step(&self, scenario: &mut Scenario) -> Result<()>;

    fn get_location(&self) -> Option<&CodeLocation>;

    fn get_pattern(&self) -> Option<&String>;

    fn get_arguments(&self) -> &Vec<Argument>;
}


#[derive(Debug)]
pub struct HookDefinitionMatch {
    pub hook_definition: HookDefinition,
    pub arguments: Vec<Argument>,
}

impl StepDefinitionMatch for HookDefinitionMatch {
    fn get_step(&self) -> &PickleStep {
        unimplemented!("HookDefinitionMatch::get_step(&self)");
    }

    fn run_step(&self, scenario: &mut Scenario) -> Result<()> {
        self.hook_definition.execute(scenario)
    }

    fn dry_run_step(&self, scenario: &mut Scenario) -> Result<()> {
        Ok(())
    }

    fn get_location(&self) -> Option<&CodeLocation> {
        Some(&self.hook_definition.get_location())
    }

    fn get_pattern(&self) -> Option<&String> {
        None
    }

    fn get_arguments(&self) -> &Vec<Argument> {
        &self.arguments
    }
}

#[derive(Debug)]
pub struct PickleStepDefinitionMatch {
    pub step_definition: StepDefinition,
    pub feature_path: String,
    pub step: Arc<PickleStep>,
    pub arguments: Vec<Argument>,
}

impl PickleStepDefinitionMatch {
    pub fn get_pattern(&self) -> &String {
        self.step_definition.get_pattern()
    }

    pub fn get_arguments(&self) -> &Vec<Argument> {
        &self.arguments
    }

    pub fn get_location(&self) -> &CodeLocation {
        self.step_definition.get_location()
    }
}

impl StepDefinitionMatch for PickleStepDefinitionMatch {
    fn get_step(&self) -> &PickleStep {
        &self.step
    }

    fn run_step(&self, scenario: &mut Scenario) -> Result<()> {
        self.step_definition.execute(scenario, Vec::new())?;
        Ok(())
    }

    fn dry_run_step(&self, scenario: &mut Scenario) -> Result<()> {
        Ok(())
    }

    fn get_location(&self) -> Option<&CodeLocation> {
        Some(&self.step_definition.get_location())
    }

    fn get_pattern(&self) -> Option<&String> {
        Some(self.step_definition.get_pattern())
    }

    fn get_arguments(&self) -> &Vec<Argument> {
        &self.arguments
    }
}

#[derive(Debug)]
pub struct AmbiguousPickleStepDefinitionMatch {
    pub feature_path: String,
    pub step: Arc<PickleStep>,
    pub arguments: Vec<Argument>,
}

impl StepDefinitionMatch for AmbiguousPickleStepDefinitionMatch {
    fn get_step(&self) -> &PickleStep {
        &self.step
    }

    fn run_step(&self, scenario: &mut Scenario) -> Result<()> {
        unimplemented!();
    }

    fn dry_run_step(&self, scenario: &mut Scenario) -> Result<()> {
        self.run_step(scenario)
    }

    fn get_location(&self) -> Option<&CodeLocation> {
        None
    }

    fn get_pattern(&self) -> Option<&String> {
        None
    }

    fn get_arguments(&self) -> &Vec<Argument> {
        &self.arguments
    }
}

#[derive(Debug)]
pub struct UndefinedPickleStepDefinitionMatch {
    pub step: Arc<PickleStep>,
    pub arguments: Vec<Argument>,
}

impl StepDefinitionMatch for UndefinedPickleStepDefinitionMatch {
    fn get_step(&self) -> &PickleStep {
        &self.step
    }

    fn run_step(&self, _scenario: &mut Scenario) -> Result<()> {
        Err(Error::UndefinedStepDefinition)
    }

    fn dry_run_step(&self, scenario: &mut Scenario) -> Result<()> {
        self.run_step(scenario)
    }

    fn get_location(&self) -> Option<&CodeLocation> {
        None
    }

    fn get_pattern(&self) -> Option<&String> {
        None
    }

    fn get_arguments(&self) -> &Vec<Argument> {
        &self.arguments
    }
}
