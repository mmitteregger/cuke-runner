use std::fmt;
use std::str::FromStr;

use {StepArgument, DataTable};

pub type FromStepArgumentResult<T> = ::std::result::Result<T, FromStepArgumentError>;
type Result<T> = FromStepArgumentResult<T>;

pub trait FromStepArgument<'a>: Sized {
    fn from_step_argument(step_argument: &'a StepArgument) -> Result<Self>;
}

/// The error holding information for a failed `FromStepArgument` conversion.
#[derive(Fail, Debug)]
pub struct FromStepArgumentError {
    pub message: String,
}

impl FromStepArgumentError {
    pub fn new<S: Into<String>>(message: S) -> FromStepArgumentError {
        FromStepArgumentError {
            message: message.into()
        }
    }
}

impl fmt::Display for FromStepArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl From<String> for FromStepArgumentError {
    fn from(message: String) -> FromStepArgumentError {
        FromStepArgumentError {
            message
        }
    }
}

impl<'a, T: FromStr> FromStepArgument<'a> for T where <T as std::str::FromStr>::Err: fmt::Debug {
    fn from_step_argument(step_argument: &'a StepArgument) -> Result<T> {
        step_argument.get_value().parse().map_err(|err| FromStepArgumentError {
            message: format!("{:?}", err),
        })
    }
}

impl<'a> FromStepArgument<'a> for &'a DataTable {
    fn from_step_argument(step_argument: &'a StepArgument) -> Result<&'a DataTable> {
        match step_argument {
            StepArgument::DataTable(ref data_table_argument) => Ok(&data_table_argument.value),
            _ => Err(FromStepArgumentError {
                message: format!("cannot get DataTable ref from step argument: {:?}", step_argument),
            }),
        }
    }
}
