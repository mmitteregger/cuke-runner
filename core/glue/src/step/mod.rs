use std::fmt;
use std::str::FromStr;

use crate::error::ExecutionError;
use crate::location::StaticGlueCodeLocation;
use crate::scenario::Scenario;
use crate::step::argument::StepArgument;

pub mod argument;

/// The type of a step handler (wraps a user defined step function).
#[doc(hidden)]
pub type StepFn = fn(&mut Scenario, &[StepArgument<'_>]) -> ::std::result::Result<(), ExecutionError>;

/// Generated info for a step definition (a `#[step(...)]` annotated function).
#[doc(hidden)]
pub struct StaticStepDef {
    /// Name of the step definition function.
    pub name: &'static str,
    /// Step definition keyword like "Given", "When" and "Then".
    pub keyword: StepKeyword,
    /// The step definition expression to match a step in a cucumber scenario.
    pub expression: &'static str,
    /// The generated step handler function that will call the user defined annotated function.
    pub step_fn: StepFn,
    /// Location of the user defined annotated function.
    pub location: StaticGlueCodeLocation,
}

impl fmt::Debug for StaticStepDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> ::std::result::Result<(), fmt::Error> {
        f.debug_struct("StaticStepDef")
            .field("name", &self.name)
            .field("keyword", &self.keyword)
            .field("expression", &self.expression)
            .field("step_fn", &"<step_fn>")
            .field("location", &self.location)
            .finish()
    }
}

/// A step keyword like "Given", "When" and "Then".
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum StepKeyword {
    Given,
    When,
    Then,
    Star,
}

impl StepKeyword {
    #[inline]
    pub fn as_str(self) -> &'static str {
        use self::StepKeyword::*;

        match self {
            Given => "Given",
            When => "When",
            Then => "Then",
            Star => "*",
        }
    }
}

impl FromStr for StepKeyword {
    type Err = ();

    fn from_str(s: &str) -> Result<StepKeyword, ()> {
        use self::StepKeyword::*;

        match s.to_lowercase().as_ref() {
            "given" => Ok(Given),
            "when" => Ok(When),
            "then" => Ok(Then),
            "*" => Ok(Star),
            _ => Err(()),
        }
    }
}

impl fmt::Display for StepKeyword {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
