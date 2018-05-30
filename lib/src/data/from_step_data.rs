use std::fmt;
use std::result;

use data::StepData;

/// A type alias for `Result<T, cuke_runner::data::FromStepDataError>`.
pub type Result<T> = result::Result<T, Error>;

pub trait FromStepData<'a>: Sized {
    fn from_step_data(step_data: &'a StepData) -> Result<Self>;
}

/// The error holding information for failed `FromStepData` conversions.
#[derive(Fail, Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new<S: Into<String>>(message: S) -> Error {
        Error {
            message: message.into()
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl From<String> for Error {
    fn from(message: String) -> Error {
        Error {
            message
        }
    }
}


impl<'a> FromStepData<'a> for &'a str {
    fn from_step_data(step_data: &'a StepData) -> Result<&'a str> {
        unimplemented!()
    }
}

impl<'a> FromStepData<'a> for String {
    fn from_step_data(step_data: &'a StepData) -> Result<String> {
        unimplemented!()
    }
}

impl<'a> FromStepData<'a> for f64 {
    fn from_step_data(step_data: &'a StepData) -> Result<f64> {
        unimplemented!()
    }
}
