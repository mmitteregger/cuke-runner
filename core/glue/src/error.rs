use std::fmt;

use crate::scenario::FromScenarioError;
use crate::step::argument::FromStepArgumentError;
use crate::panic::{self, PanicInfo};

#[derive(Fail, Debug)]
pub enum ExecutionError {
    /// An error that occurred while converting scenario data to a step function parameter.
    FromScenario(#[cause] FromScenarioError),
    FromStepArgument(#[cause] FromStepArgumentError),
    Panic(PanicError),
    Other(#[cause] ::failure::Error),
}

#[derive(Fail, Debug)]
pub struct PanicError {
    panic_info: PanicInfo,
}

impl PanicError {
    #[doc(hidden)]
    pub fn new() -> PanicError {
        let cuke_panic_info = panic::remove_current_panic_info()
            .expect("could not find current panic info");

        PanicError {
            panic_info: cuke_panic_info,
        }
    }

    pub fn panic_info(&self) -> &PanicInfo {
        &self.panic_info
    }
}

impl fmt::Display for PanicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.panic_info.short_display())
    }
}

impl From<FromScenarioError> for ExecutionError {
    fn from(err: FromScenarioError) -> ExecutionError {
        ExecutionError::FromScenario(err)
    }
}

impl From<FromStepArgumentError> for ExecutionError {
    fn from(err: FromStepArgumentError) -> ExecutionError {
        ExecutionError::FromStepArgument(err)
    }
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecutionError::FromScenario(ref err) => fmt::Display::fmt(err, f),
            ExecutionError::FromStepArgument(ref err) => fmt::Display::fmt(err, f),
            ExecutionError::Panic(ref err) => fmt::Display::fmt(err, f),
            ExecutionError::Other(ref err) => fmt::Display::fmt(err, f),
        }
    }
}
