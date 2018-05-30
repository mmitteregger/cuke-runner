use std::ops::{Deref, DerefMut};

use data::{StepData, FromStepData, FromStepDataResult};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State<'a, T: 'static>(&'a mut T);

//impl<'a, T: 'static> State<'a, T> {
//    #[inline(always)]
//    pub fn inner(&self) -> &'a T {
//        self.0
//    }
//}

impl<'a, T> FromStepData<'a> for State<'a, T> {
    fn from_step_data(step_data: &'a StepData) -> FromStepDataResult<State<'a, T>> {
        unimplemented!()
    }
}

impl<'a, T: 'static> Deref for State<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        self.0
    }
}

impl<'a, T: 'static> DerefMut for State<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
