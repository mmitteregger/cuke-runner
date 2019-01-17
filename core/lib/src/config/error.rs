use std::fmt;
use std::io;
use std::env;

/// The specific type of an error.
#[derive(Fail, Debug)]
pub enum Error {
    /// An I/O error that occurred while reading the cukes config.
    Io(#[cause] io::Error),
    /// A required environment variable (from cargo) cannot be used.
    EnvVar(#[cause] env::VarError, &'static str),
    /// The configuration file was not found.
    NotFound,
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::EnvVar(ref err, ref var) =>
                write!(f, "required env variable \"{}\" could not be used: {}", var, err),
            Error::NotFound => write!(f, "config file was not found"),
            Error::__Nonexhaustive => unreachable!(),
        }
    }
}
