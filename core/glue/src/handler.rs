use std::fmt;

use {Scenario, FromScenarioError};

#[derive(Fail, Debug)]
pub enum ExecutionError {
    /// An error that occurred while converting scenario data to a step function parameter.
    FromScenario(#[cause] FromScenarioError),
    Panic(#[cause] PanicError),
    Other(#[cause] ::failure::Error),
}

pub fn panic_error(error: Box<::std::any::Any + Send + 'static>) -> ExecutionError {
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

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecutionError::FromScenario(ref err) => fmt::Display::fmt(err, f),
            ExecutionError::Panic(ref err) => fmt::Display::fmt(err, f),
            ExecutionError::Other(ref err) => fmt::Display::fmt(err, f),
        }
    }
}

/// The type of a generated hook handler (wraps a user defined hook function).
pub type HookFn = fn() -> ::std::result::Result<(), ExecutionError>;

/// The type of a step handler (wraps a user defined step function).
pub type StepFn = fn(&mut Scenario) -> ::std::result::Result<(), ExecutionError>;
