use std::fmt;
use std::str::FromStr;

use crate::CodeLocation;
use crate::scenario::Scenario;
use crate::error::ExecutionError;

/// The type of a generated hook handler (wraps a user defined hook function).
pub type HookFn = fn(&mut Scenario) -> ::std::result::Result<(), ExecutionError>;

/// Generated info for a hook definition
/// (for example a `#[before_scenario(...)]` annotated function).
pub struct StaticHookDef {
    /// Name of the step definition function.
    pub name: &'static str,
    /// Execution order of the hook definition function of the same hook type. Higher
    pub order: isize,
    /// The generated step handler function responsible for calling the step definition function.
    pub hook_fn: HookFn,
    /// The generated step handler function responsible for calling the step definition function.
    pub location: CodeLocation,
}

impl fmt::Debug for StaticHookDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        f.debug_struct("StaticHookDef")
            .field("name", &self.name)
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
