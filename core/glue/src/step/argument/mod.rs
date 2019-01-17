mod expression;
mod doc_string;
mod data_table;

pub use self::expression::Expression;
pub use self::doc_string::DocString;
pub use self::data_table::{DataTable, FromDataTableRow};

use std::fmt;
use std::str::FromStr;

/// An argument from a scenario step that is passed to an step function.
///
/// The lifetime parameter `'s` refers to the lifetime of the step.
/// It cannot escape the step function.
#[derive(Debug)]
pub enum StepArgument<'s> {
    Expression(Expression<'s>),
    DocString(DocString<'s>),
    DataTable(DataTable<'s>),
}

pub type FromStepArgumentResult<T> = ::std::result::Result<T, FromStepArgumentError>;

/// Converts a `StepArgument` to `Self`.
///
/// The lifetime parameter `'s` refers to the lifetime of the step.
/// It cannot escape the step function.
pub trait FromStepArgument<'s>: Sized {
    fn from_step_argument(step_argument: &'s StepArgument) -> FromStepArgumentResult<Self>;
}

/// The error holding information for a failed `FromStepArgument` conversion.
#[derive(Fail, Debug)]
pub struct FromStepArgumentError {
    message: String,
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

impl<'s, T: FromStr> FromStepArgument<'s> for T where <T as std::str::FromStr>::Err: fmt::Debug {
    fn from_step_argument(step_argument: &'s StepArgument) -> FromStepArgumentResult<T> {
        let str_value = match step_argument {
            StepArgument::Expression(expression) => Some(expression.value()),
            StepArgument::DocString(doc_string) => Some(doc_string.value()),
            StepArgument::DataTable(_data_table) => None,
        };

        match str_value {
            Some(value) => value.parse()
                .map_err(|err| FromStepArgumentError::new(format!("{:?}", err))),
            None => {
                Err(FromStepArgumentError::new(
                    "cannot parse DataTable, use DataTable itself as argument type".to_owned()
                ))
            },
        }
    }
}

impl<'s> FromStepArgument<'s> for &'s DataTable<'s> {
    fn from_step_argument(step_argument: &'s StepArgument) -> FromStepArgumentResult<&'s DataTable<'s>> {
        match step_argument {
            StepArgument::DataTable(ref data_table) => Ok(&data_table),
            _ => Err(FromStepArgumentError {
                message: format!("cannot get DataTable ref from step argument: {:?}", step_argument),
            }),
        }
    }
}
