use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Default)]
pub struct Scenario {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Scenario {
    #[doc(hidden)]
    pub fn new() -> Scenario {
        Scenario {
            data: HashMap::new(),
        }
    }

    pub fn set<T: 'static>(&mut self, data: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(data));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.data.get(&TypeId::of::<T>())
            .map(|value| value.downcast_ref::<T>().unwrap())
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.data.get_mut(&TypeId::of::<T>())
            .map(|value| value.downcast_mut::<T>().unwrap())
    }
}

pub type FromScenarioResult<T> = ::std::result::Result<T, FromScenarioError>;

pub trait FromScenario<'a>: Sized {
    fn from_scenario(scenario: &'a Scenario) -> FromScenarioResult<Self>;
}

pub trait FromScenarioMut<'a>: Sized {
    fn from_scenario_mut(scenario: &'a mut Scenario) -> FromScenarioResult<Self>;
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

impl<'a> FromScenario<'a> for &'a Scenario {
    fn from_scenario(scenario: &'a Scenario) -> FromScenarioResult<&'a Scenario> {
        Ok(scenario)
    }
}

impl<'a> FromScenarioMut<'a> for &'a mut Scenario {
    fn from_scenario_mut(scenario: &'a mut Scenario) -> FromScenarioResult<&'a mut Scenario> {
        Ok(scenario)
    }
}

impl<'a, T: 'static> FromScenario<'a> for Option<&'a T> {
    fn from_scenario(scenario: &'a Scenario) -> FromScenarioResult<Option<&'a T>> {
        Ok(scenario.get::<T>())
    }
}

impl<'a, T: 'static> FromScenarioMut<'a> for Option<&'a mut T> {
    fn from_scenario_mut(scenario: &'a mut Scenario) -> FromScenarioResult<Option<&'a mut T>> {
        Ok(scenario.get_mut::<T>())
    }
}
