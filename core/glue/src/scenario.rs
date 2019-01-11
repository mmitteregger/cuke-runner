use std::fmt;
use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scenario {
    user_data: HashMap<TypeId, Box<Any>>,
}

impl Scenario {
    pub fn new() -> Scenario {
        Scenario {
            user_data: HashMap::new(),
        }
    }

    pub fn set_user_data<T: 'static>(&mut self, user_data: T) {
        self.user_data.insert(TypeId::of::<T>(), Box::new(user_data));
    }

    pub fn get_user_data<T: 'static>(&mut self) -> Option<&mut T> {
        self.user_data.get_mut(&TypeId::of::<T>())
            .map(|value| value.downcast_mut::<T>().unwrap())
    }
}

pub type FromScenarioResult<T> = ::std::result::Result<T, FromScenarioError>;

pub trait FromScenario<'a>: Sized {
    fn from_scenario(scenario: &'a mut Scenario) -> FromScenarioResult<Self>;
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

impl<'a> FromScenario<'a> for &'a mut Scenario {
    fn from_scenario(scenario: &'a mut Scenario) -> FromScenarioResult<&'a mut Scenario> {
        Ok(scenario)
    }
}
