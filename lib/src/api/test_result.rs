use std::time::Duration;

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
        unimplemented!("get_error_message() should include a backtrace");
        self.error.as_ref().map(|error| error.to_string())
    }

    pub fn get_error(&self) -> Option<&Error> {
        self.error.as_ref()
    }
}
