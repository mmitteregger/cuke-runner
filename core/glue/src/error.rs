use std::fmt;

use crate::scenario::FromScenarioError;
use crate::step::argument::FromStepArgumentError;

#[derive(Fail, Debug)]
pub enum ExecutionError {
    /// An error that occurred while converting scenario data to a step function parameter.
    FromScenario(#[cause] FromScenarioError),
    FromStepArgument(#[cause] FromStepArgumentError),
    Panic(#[cause] PanicError),
    Other(#[cause] ::failure::Error),
}

pub fn panic_error(error: Box<dyn (::std::any::Any) + Send + 'static>) -> ExecutionError {
    ExecutionError::Panic(PanicError { message: format!("{:?}", error) })
}

#[derive(Fail, Debug)]
pub struct PanicError {
    message: String,
}

impl fmt::Display for PanicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.message, f)
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
