use std::fmt;
use std::result;
use std::io;

use gherkin;
use walkdir;
use failure::Fail;

use crate::config;
use crate::glue;

/// A type alias for `Result<T, cuke_runner::Error>`.
pub type Result<T> = result::Result<T, Error>;

/// The specific type of an error.
#[derive(Fail, Debug)]
pub enum Error {
    /// An io error that occurred while reading feature files.
    Io(#[cause] io::Error),
    /// An error that occurred while reading the cukes config.
    Config(#[cause] config::Error),
    /// An error that occurred while traversing the features directory.
    WalkDir(#[cause] walkdir::Error),
    /// An error that occurred while parsing a feature file.
    Gherkin(#[cause] gherkin::Error),
    /// An error that occurred while converting scenario data to a step function parameter.
    FromScenario(#[cause] glue::scenario::FromScenarioError),
    /// An error that occurred while executing a step or hook function.
    Execution(crate::glue::error::ExecutionError),
    AmbiguousStepDefinitions,
    UndefinedStepDefinition,
    Pending,
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<config::Error> for Error {
    fn from(err: config::Error) -> Error {
        Error::Config(err)
    }
}

impl From<walkdir::Error> for Error {
    fn from(err: walkdir::Error) -> Error {
        Error::WalkDir(err)
    }
}

impl From<gherkin::Error> for Error {
    fn from(err: gherkin::Error) -> Error {
        Error::Gherkin(err)
    }
}

impl From<glue::scenario::FromScenarioError> for Error {
    fn from(err: glue::scenario::FromScenarioError) -> Error {
        Error::FromScenario(err)
    }
}

impl From<glue::error::ExecutionError> for Error {
    fn from(err: glue::error::ExecutionError) -> Error {
        Error::Execution(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "{}", err),
            Error::Config(ref err) => write!(f, "{}", err),
            Error::WalkDir(ref err) => write!(f, "{}", err),
            Error::Gherkin(ref err) => write!(f, "{}", err),
            Error::FromScenario(ref err) => write!(f, "{}", err),
            Error::Execution(ref err) => write!(f, "{}", err),
            Error::AmbiguousStepDefinitions => write!(f, "AmbiguousStepDefinitions"),
            Error::UndefinedStepDefinition => write!(f, "UndefinedStepDefinition"),
            Error::Pending => write!(f, "TODO: implement me"),
            Error::__Nonexhaustive => unreachable!(),
        }
    }
}
