use std::fmt::Debug;

use gherkin::pickle::PickleStep;

use api::CodeLocation;
use glue::step::argument::StepArgument;

/// A test step can either represent the execution of a hook or a pickle step.
/// Each step is tied to some glue code.
#[derive(Debug)]
pub enum TestStep<'s> {
    Hook(&'s HookTestStep<'s>),
    Pickle(&'s PickleStepTestStep<'s>),
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum HookType {
    BeforeScenario,
    AfterScenario,
    BeforeStep,
    AfterStep,
}

/// Hooks are invoked before and after each scenario and before and after each gherkin step in a scenario.
pub trait HookTestStep<'s>: Debug + Send + Sync {
    /// Representation of the source code location of the glue.
    fn get_code_location(&self) -> Option<&CodeLocation>;

    /// The hook hook type (BeforeScenario, AfterScenario, ...).
    fn get_hook_type(&self) -> HookType;
}

/// A pickle test step matches a line in a Gherkin scenario or background.
pub trait PickleStepTestStep<'s>: Debug + Send + Sync {
    /// Representation of the source code location of the glue.
    fn get_code_location(&self) -> Option<&CodeLocation>;

    /// The pattern or expression used to match the glue code to the Gherkin step.
    fn get_pattern(&self) -> Option<&String>;

    /// The matched Gherkin step as a compiled Pickle.
    fn get_pickle_step(&self) -> &PickleStep;

    /// Returns arguments provided to the Gherkin step.
    fn get_arguments(&self) -> &[StepArgument];

    /// The line in the feature file defining this step.
    fn get_step_line(&self) -> u32;

    /// A uri to to the feature and line of this step.
    fn get_step_location(&self) -> String;

    /// The full text of the Gherkin step.
    fn get_step_text(&self) -> &String;
}
