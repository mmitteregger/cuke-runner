use std::fmt;
use std::str::FromStr;

use crate::error::ExecutionError;
use crate::location::StaticGlueCodeLocation;
use crate::scenario::Scenario;

/// The type of a generated hook handler (wraps a user defined hook function).
#[doc(hidden)]
pub type HookFn = fn(&mut Scenario) -> ::std::result::Result<(), ExecutionError>;

/// Generated info for a hook definition
/// (for example a `#[before_scenario(...)]` annotated function).
#[doc(hidden)]
pub struct StaticHookDef {
    /// Name of the step definition function.
    pub name: &'static str,
    /// Execution order of the hook definition function of the same hook type.
    ///
    /// Before hooks are executed in ascending order (..., -1, 0, 1, ...) and
    /// after hooks are executed in descending order (..., 1, 0, -1, ...).
    pub order: isize,
    /// A tag expression is an infix boolean expression
    /// to restrict when the hook should be executed.
    ///
    /// # Examples
    ///
    /// ```text ignore
    /// | Expression         | Description                                                   |
    /// |--------------------|---------------------------------------------------------------|
    /// | @fast              | Scenarios tagged with @fast                                   |
    /// | @wip and not @slow | Scenarios tagged with @wip that aren’t also tagged with @slow |
    /// | @smoke and @fast   | Scenarios tagged with both @smoke and @fast                   |
    /// | @gui or @database  | Scenarios tagged with either @gui or @database                |
    /// ```
    /// For even more advanced tag expressions you can use parenthesis for clarity,
    /// or to change operator precedence:
    /// ```text ignore
    /// (@smoke or @ui) and (not @slow)
    /// ```
    pub tag_expression: &'static str,
    /// The generated hook handler function that will call the user defined annotated function.
    pub hook_fn: HookFn,
    /// Location of the user defined annotated function.
    pub location: StaticGlueCodeLocation,
}

impl fmt::Debug for StaticHookDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        f.debug_struct("StaticHookDef")
            .field("name", &self.name)
            .field("tag_expression", &self.tag_expression)
            .field("hook_fn", &"<hook_fn>")
            .field("location", &self.location)
            .finish()
    }
}

/// A hook type like "BeforeScenario" or "AfterStep".
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum HookType {
    BeforeScenario,
    BeforeStep,
    AfterStep,
    AfterScenario,
}

impl HookType {
    #[inline]
    pub fn as_str(self) -> &'static str {
        use self::HookType::*;

        match self {
            BeforeScenario => "BeforeScenario",
            BeforeStep => "BeforeStep",
            AfterStep => "AfterStep",
            AfterScenario => "AfterScenario",
        }
    }
}

impl FromStr for HookType {
    type Err = ();

    fn from_str(s: &str) -> Result<HookType, ()> {
        use self::HookType::*;

        match s {
            "BeforeScenario" => Ok(BeforeScenario),
            "BeforeStep" => Ok(BeforeStep),
            "AfterStep" => Ok(AfterStep),
            "AfterScenario" => Ok(AfterScenario),
            _ => Err(()),
        }
    }
}

impl fmt::Display for HookType {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}
