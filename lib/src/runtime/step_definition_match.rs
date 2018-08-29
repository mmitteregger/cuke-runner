use std::fmt::Debug;
use std::sync::Arc;

use gherkin::pickle::PickleStep;

use error::{Result, Error};
use api::SourceCodeLocation;
use runtime::{Argument, HookDefinition, StepDefinition};
use runtime::Scenario;

pub trait StepDefinitionMatch: Debug + Send + Sync {
    fn get_step(&self) -> &PickleStep;

    fn run_step(&self, language: &str, scenario: &mut Scenario) -> Result<()>;

    fn dry_run_step(&self, language: &str, scenario: &mut Scenario) -> Result<()>;

    fn get_location(&self) -> &SourceCodeLocation;

    fn get_pattern(&self) -> Option<&String>;
}


#[derive(Debug)]
pub struct HookDefinitionMatch {
    pub hook_definition: HookDefinition,
}

impl StepDefinitionMatch for HookDefinitionMatch {
    fn get_step(&self) -> &PickleStep {
        unimplemented!("HookDefinitionMatch::get_step(&self)");
    }

    fn run_step(&self, language: &str, scenario: &mut Scenario) -> Result<()> {
        self.hook_definition.execute(scenario)
    }

    fn dry_run_step(&self, language: &str, scenario: &mut Scenario) -> Result<()> {
        Ok(())
    }

    fn get_location(&self) -> &SourceCodeLocation {
        self.hook_definition.get_location()
    }

    fn get_pattern(&self) -> Option<&String> {
        None
    }
}

#[derive(Debug)]
pub struct PickleStepDefinitionMatch {
    pub arguments: Vec<Box<Argument>>,
    pub step_definition: StepDefinition,
    pub feature_path: String,
    pub step: Arc<PickleStep>,
}

impl PickleStepDefinitionMatch {
    pub fn get_pattern(&self) -> &String {
        self.step_definition.get_pattern()
    }

    pub fn get_arguments(&self) -> &Vec<Box<Argument>> {
        &self.arguments
    }

    pub fn get_location(&self) -> &SourceCodeLocation {
        self.step_definition.get_location()
    }
}

impl StepDefinitionMatch for PickleStepDefinitionMatch {
    fn get_step(&self) -> &PickleStep {
        &self.step
    }

    fn run_step(&self, language: &str, scenario: &mut Scenario) -> Result<()> {
        unimplemented!();
    }

    fn dry_run_step(&self, language: &str, scenario: &mut Scenario) -> Result<()> {
        Ok(())
    }

    fn get_location(&self) -> &SourceCodeLocation {
        self.step_definition.get_location()
    }

    fn get_pattern(&self) -> Option<&String> {
        Some(self.step_definition.get_pattern())
    }
}

#[derive(Debug)]
pub struct AmbiguousPickleStepDefinitionMatch {
    pub feature_path: String,
    pub step: Arc<PickleStep>,
}

impl StepDefinitionMatch for AmbiguousPickleStepDefinitionMatch {
    fn get_step(&self) -> &PickleStep {
        &self.step
    }

    fn run_step(&self, language: &str, scenario: &mut Scenario) -> Result<()> {
        unimplemented!();
    }

    fn dry_run_step(&self, language: &str, scenario: &mut Scenario) -> Result<()> {
        self.run_step(language, scenario)
    }

    fn get_location(&self) -> &SourceCodeLocation {
        unimplemented!();
    }

    fn get_pattern(&self) -> Option<&String> {
        None
    }
}

#[derive(Debug)]
pub struct UndefinedPickleStepDefinitionMatch {
    pub step: Arc<PickleStep>,
}

impl StepDefinitionMatch for UndefinedPickleStepDefinitionMatch {
    fn get_step(&self) -> &PickleStep {
        &self.step
    }

    fn run_step(&self, _language: &str, _scenario: &mut Scenario) -> Result<()> {
        Err(Error::UndefinedStepDefinition)
    }

    fn dry_run_step(&self, language: &str, scenario: &mut Scenario) -> Result<()> {
        self.run_step(language, scenario)
    }

    fn get_location(&self) -> &SourceCodeLocation {
        unimplemented!();
    }

    fn get_pattern(&self) -> Option<&String> {
        None
    }
}
