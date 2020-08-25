use std::fmt::Debug;

use gherkin::cuke;

use crate::api::GlueCodeLocation;
use crate::glue::step::argument::StepArgument;

/// A test step can either represent the execution of a hook or a cuke step.
/// Each step is tied to some glue code.
#[derive(Debug)]
pub enum TestStep<'a, 's> {
    Hook(&'a dyn HookTestStep<'s>),
    Cuke(&'a dyn CukeStepTestStep<'s>),
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
    fn get_glue_code_location(&self) -> Option<&GlueCodeLocation>;

    /// The hook hook type (BeforeScenario, AfterScenario, ...).
    fn get_hook_type(&self) -> HookType;
}

/// A cuke test step matches a line in a Gherkin scenario or background.
pub trait CukeStepTestStep<'s>: Debug + Send + Sync {
    /// Representation of the source code location of the glue.
    fn get_glue_code_location(&self) -> Option<&GlueCodeLocation>;

    /// The pattern or expression used to match the glue code to the Gherkin step.
    fn get_pattern(&self) -> Option<&str>;

    /// The matched Gherkin step.
    fn get_cuke_step(&self) -> &cuke::Step<'_>;

    /// Returns arguments provided to the Gherkin step.
    fn get_arguments(&self) -> &[StepArgument<'_>];

    /// The keyword of the Gherkin step like "Given ", "When " or "Then ".
    fn get_step_keyword(&self) -> &str;

    /// The line in the feature file defining this step.
    fn get_step_line(&self) -> u32;

    /// A uri to to the feature and line of this step.
    fn get_step_location(&self) -> String;

    /// The full text of the Gherkin step.
    fn get_step_text(&self) -> &str;

    /// Whether this step is part of a Background definition.
    fn is_background_step(&self) -> bool;
}
