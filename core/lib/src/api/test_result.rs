use std::time::Duration;
use std::fmt;

use failure::{Fail, AsFail};

use error::Error;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub enum TestResultStatus {
    Passed,
    Skipped,
    Pending,
    Undefined,
    Ambiguous,
    Failed,
}

impl TestResultStatus {
    pub fn is_ok(&self, strict: bool) -> bool {
        self.has_always_ok_status() || (!strict && self.has_ok_when_not_strict_status())
    }

    fn has_always_ok_status(&self) -> bool {
        self.eq(&TestResultStatus::Passed) || self.eq(&TestResultStatus::Skipped)
    }

    fn has_ok_when_not_strict_status(&self) -> bool {
        self.eq(&TestResultStatus::Undefined) || self.eq(&TestResultStatus::Pending)
    }

    pub fn ansi_color_code(&self) -> u8 {
        use self::TestResultStatus::*;

        match *self {
            Passed => 32 /* green */,
            Skipped => 36 /* cyan */,
            Pending => 33 /* yellow */,
            Undefined => 33 /* yellow */,
            Ambiguous => 31 /* red */,
            Failed => 31 /* red */,
        }
    }
}

impl fmt::Display for TestResultStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TestResultStatus::*;

        match *self {
            Passed => write!(f, "Passed"),
            Skipped => write!(f, "Skipped"),
            Pending => write!(f, "Pending"),
            Undefined => write!(f, "Undefined"),
            Ambiguous => write!(f, "Ambiguous"),
            Failed => write!(f, "Failed"),
        }
    }
}

/// The result of a step or scenario.
#[derive(Debug)]
pub struct TestResult {
    pub status: TestResultStatus,
    pub duration: Option<Duration>,
    pub error: Option<Error>,
}

impl TestResult {
    pub fn get_status(&self) -> TestResultStatus {
        self.status
    }

    pub fn get_duration(&self) -> Option<Duration> {
        self.duration
    }

    pub fn get_error_message(&self) -> Option<String> {
        self.error.as_ref().map(|error| {
            let mut error_message = String::new();

            let fail = error.as_fail();
            error_message.push_str(&format!("{}", fail));

            if let Some(backtrace) = error.backtrace() {
                error_message.push_str(&format!("\n{}", backtrace));
            }

            for cause in fail.iter_causes() {
                error_message.push_str(&format!("\ncaused by: {}", cause));

                if let Some(backtrace) = cause.backtrace() {
                    error_message.push_str(&format!("\n{}", backtrace));
                }
            }

            error_message
        })
    }

    pub fn get_error(&self) -> Option<&Error> {
        self.error.as_ref()
    }
}
