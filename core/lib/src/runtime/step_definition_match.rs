use gherkin::cuke;
use glue::step::argument::StepArgument;

use error::{Result, Error};
use api::CodeLocation;
use runtime::{HookDefinition, StepDefinition};
use runtime::Scenario;

#[derive(Debug)]
pub enum StepDefinitionMatch<'s> {
    Hook(HookDefinitionMatch<'s>),
    Cuke(CukeStepDefinitionMatch<'s>),
    Ambiguous(AmbiguousCukeStepDefinitionMatch<'s>),
    Undefined(UndefinedCukeStepDefinitionMatch<'s>),
}

impl<'s> StepDefinitionMatch<'s> {
    pub fn get_step(&self) -> &cuke::Step {
        match self {
            StepDefinitionMatch::Hook(hook) => hook.get_step(),
            StepDefinitionMatch::Cuke(cuke_step) => cuke_step.get_step(),
            StepDefinitionMatch::Ambiguous(cuke_step) => cuke_step.get_step(),
            StepDefinitionMatch::Undefined(cuke_step) => cuke_step.get_step(),
        }
    }

    pub fn run_step(&self, scenario: &mut Scenario) -> Result<()> {
        match self {
            StepDefinitionMatch::Hook(hook) => hook.run_step(scenario),
            StepDefinitionMatch::Cuke(cuke_step) => cuke_step.run_step(scenario),
            StepDefinitionMatch::Ambiguous(cuke_step) => cuke_step.run_step(scenario),
            StepDefinitionMatch::Undefined(cuke_step) => cuke_step.run_step(scenario),
        }
    }

    pub fn dry_run_step(&self, scenario: &mut Scenario) -> Result<()> {
        match self {
            StepDefinitionMatch::Hook(hook) => hook.dry_run_step(scenario),
            StepDefinitionMatch::Cuke(cuke_step) => cuke_step.dry_run_step(scenario),
            StepDefinitionMatch::Ambiguous(cuke_step) => cuke_step.dry_run_step(scenario),
            StepDefinitionMatch::Undefined(cuke_step) => cuke_step.dry_run_step(scenario),
        }
    }

    pub fn get_location(&self) -> Option<&CodeLocation> {
        match self {
            StepDefinitionMatch::Hook(hook) => hook.get_location(),
            StepDefinitionMatch::Cuke(cuke_step) => cuke_step.get_location(),
            StepDefinitionMatch::Ambiguous(cuke_step) => cuke_step.get_location(),
            StepDefinitionMatch::Undefined(cuke_step) => cuke_step.get_location(),
        }
    }

    pub fn get_pattern(&self) -> Option<&String> {
        match self {
            StepDefinitionMatch::Hook(hook) => hook.get_pattern(),
            StepDefinitionMatch::Cuke(cuke_step) => cuke_step.get_pattern(),
            StepDefinitionMatch::Ambiguous(cuke_step) => cuke_step.get_pattern(),
            StepDefinitionMatch::Undefined(cuke_step) => cuke_step.get_pattern(),
        }
    }

    pub fn get_arguments(&self) -> &[StepArgument] {
        match self {
            StepDefinitionMatch::Hook(hook) => hook.get_arguments(),
            StepDefinitionMatch::Cuke(cuke_step) => cuke_step.get_arguments(),
            StepDefinitionMatch::Ambiguous(cuke_step) => cuke_step.get_arguments(),
            StepDefinitionMatch::Undefined(cuke_step) => cuke_step.get_arguments(),
        }
    }
}


#[derive(Debug)]
pub struct HookDefinitionMatch<'s> {
    pub hook_definition: HookDefinition,
    pub arguments: Vec<StepArgument<'s>>,
}

impl<'s> HookDefinitionMatch<'s> {
    pub fn get_step(&self) -> &cuke::Step {
        unimplemented!("HookDefinitionMatch::get_step(&self)");
    }

    pub fn run_step(&self, scenario: &mut Scenario) -> Result<()> {
        self.hook_definition.execute(scenario)
    }

    pub fn dry_run_step(&self, _scenario: &mut Scenario) -> Result<()> {
        Ok(())
    }

    pub fn get_location(&self) -> Option<&CodeLocation> {
        Some(&self.hook_definition.get_location())
    }

    pub fn get_pattern(&self) -> Option<&String> {
        None
    }

    pub fn get_arguments(&self) -> &[StepArgument] {
        &self.arguments
    }
}

#[derive(Debug)]
pub struct CukeStepDefinitionMatch<'s> {
    pub step_definition: StepDefinition,
    pub feature_path: String,
    pub step: &'s cuke::Step<'s>,
    pub arguments: Vec<StepArgument<'s>>,
}

impl<'s> CukeStepDefinitionMatch<'s> {
    fn get_step(&self) -> &cuke::Step {
        &self.step
    }

    fn run_step(&self, scenario: &mut Scenario) -> Result<()> {
        self.step_definition.execute(scenario, &self.arguments)?;
        Ok(())
    }

    fn dry_run_step(&self, _scenario: &mut Scenario) -> Result<()> {
        Ok(())
    }

    fn get_location(&self) -> Option<&CodeLocation> {
        Some(&self.step_definition.get_location())
    }

    fn get_pattern(&self) -> Option<&String> {
        Some(self.step_definition.get_pattern())
    }

    fn get_arguments(&self) -> &[StepArgument] {
        &self.arguments
    }
}

#[derive(Debug)]
pub struct AmbiguousCukeStepDefinitionMatch<'s> {
    pub feature_path: String,
    pub step: &'s cuke::Step<'s>,
    pub arguments: Vec<StepArgument<'s>>,
}

impl<'s> AmbiguousCukeStepDefinitionMatch<'s> {
    fn get_step(&self) -> &cuke::Step {
        &self.step
    }

    fn run_step(&self, _scenario: &mut Scenario) -> Result<()> {
        unimplemented!("AmbiguousPickleStepDefinitionMatch::run_step");
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

    fn get_arguments(&self) -> &[StepArgument] {
        &self.arguments
    }
}

#[derive(Debug)]
pub struct UndefinedCukeStepDefinitionMatch<'s> {
    pub step: &'s cuke::Step<'s>,
    pub arguments: Vec<StepArgument<'s>>,
}

impl<'s> UndefinedCukeStepDefinitionMatch<'s> {
    fn get_step(&self) -> &cuke::Step {
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

    fn get_arguments(&self) -> &[StepArgument] {
        &self.arguments
    }
}
