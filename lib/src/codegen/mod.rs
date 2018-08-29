use std::fmt;

use data::{StepData, StepKeyword};
use error::Result;

/// The type of a generated hook handler (wraps a user defined hook function).
pub type HookHandler = fn() -> Result<()>;

/// The type of a step handler (wraps a user defined step function).
pub type StepHandler = fn(&StepData) -> Result<()>;

/// Generated info for a step definition (a `#[step(...)]` annotated function).
pub struct StaticStepDefinition {
    /// Name of the step definition function.
    pub name: &'static str,
    /// Step definition keyword like "Given", "When" and "Then".
    pub keyword: StepKeyword,
    /// The step definition text to match a step in a cucumber scenario.
    pub text: &'static str,
    /// The generated handler responsible for calling the step definition function.
    pub handler: StepHandler,
}

impl fmt::Debug for StaticStepDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        f.debug_struct("StaticStepDefinition")
            .field("name", &self.name)
            .field("keyword", &self.keyword)
            .field("text", &self.text)
            .field("handler", &"<step_handler_function>")
            .finish()
    }
}
