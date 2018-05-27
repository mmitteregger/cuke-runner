use std::fmt;

use Scenario;

pub type FromScenarioResult<T> = ::std::result::Result<T, FromScenarioError>;
type Result<T> = FromScenarioResult<T>;

pub trait FromScenario<'a>: Sized {
    fn from_scenario(scenario: &'a mut Scenario) -> Result<Self>;
}

/// The error holding information for a failed `FromScenario` conversion.
#[derive(Fail, Debug)]
pub struct FromScenarioError {
    pub message: String,
}

impl FromScenarioError {
    pub fn new<S: Into<String>>(message: S) -> FromScenarioError {
        FromScenarioError {
            message: message.into()
        }
    }
}

impl fmt::Display for FromScenarioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl From<String> for FromScenarioError {
    fn from(message: String) -> FromScenarioError {
        FromScenarioError {
            message
        }
    }
}


impl<'a> FromScenario<'a> for &'a str {
    fn from_scenario(_scenario: &'a mut Scenario) -> Result<&'a str> {
        unimplemented!()
    }
}

impl<'a> FromScenario<'a> for String {
    fn from_scenario(_scenario: &'a mut Scenario) -> Result<String> {
        unimplemented!()
    }
}

impl<'a> FromScenario<'a> for f64 {
    fn from_scenario(_scenario: &'a mut Scenario) -> Result<f64> {
        unimplemented!()
    }
}
