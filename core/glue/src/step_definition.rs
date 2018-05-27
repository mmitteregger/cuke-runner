use std::fmt;

use {StepKeyword, StepFn};

/// Generated info for a step definition (a `#[step(...)]` annotated function).
pub struct StaticStepDefinition {
    /// Name of the step definition function.
    pub name: &'static str,
    /// Step definition keyword like "Given", "When" and "Then".
    pub keyword: StepKeyword,
    /// The step definition expression to match a step in a cucumber scenario.
    pub expression: &'static str,
    /// The generated step handler function responsible for calling the step definition function.
    pub step_fn: StepFn,
}

impl fmt::Debug for StaticStepDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        f.debug_struct("StaticStepDefinition")
            .field("name", &self.name)
            .field("keyword", &self.keyword)
            .field("expression", &self.expression)
            .field("step_fn", &"<step_fn>")
            .finish()
    }
}
