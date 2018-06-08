//! Scenario scoped state

use std::ops::{Deref, DerefMut};
use std::cell::{RefCell, RefMut};

use ::state::Container;

use data::{StepData, FromStepData, FromStepDataResult, FromStepDataError};


lazy_static! {
    static ref CONTAINER: Container = Container::new();
}

#[derive(Debug)]
pub struct State<'a, T: Send + 'static>(RefMut<'a, T>);

impl<'a, T: Send + 'static> State<'a, T> {
    pub fn init<F>(state_init: F) where F: Fn() -> T + 'static {
        CONTAINER.set_local(move || RefCell::new(state_init()));
    }

    pub fn get() -> RefMut<'a, T> {
        match CONTAINER.try_get_local::<RefCell<T>>() {
            Some(state) => state.borrow_mut(),
            None => panic!("Attempted to retrieve uninitialized state! \
                Call State::init(data) in a #[before] annotation function."),
        }
    }
}

impl<'a, T: Send + 'static> FromStepData<'a> for State<'a, T> {
    fn from_step_data(_step_data: &'a StepData) -> FromStepDataResult<State<'a, T>> {
        match CONTAINER.try_get_local::<RefCell<T>>() {
            Some(state) => Ok(State(state.borrow_mut())),
            None => Err(FromStepDataError::new(
                "Attempted to retrieve uninitialized state! \
                Call State::init(data) in a #[before] annotation function.")),
        }
    }
}

impl<'a, T: Send + 'static> Deref for State<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<'a, T: Send + 'static> DerefMut for State<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
