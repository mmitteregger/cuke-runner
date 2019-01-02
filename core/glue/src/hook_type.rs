//! Step keywords like "Given", "When" and "Then".

use std::fmt;
use std::str::FromStr;

use self::HookType::*;

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
    pub fn as_str(&self) -> &'static str {
        match *self {
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
