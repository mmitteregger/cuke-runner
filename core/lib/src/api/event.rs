use std::fmt::Debug;
use std::time::SystemTime;

use gherkin::pickle::PickleLocation;

use api::TestStep;
use api::TestResult;
use api::TestCase;

/// An execution event that is generated by cucumber.
#[derive(Debug)]
pub enum Event<'a> {
    /// Sent before any execution of test steps is started.
    ///
    /// This is the first event that will be sent.
    TestRunStarted {
        time: SystemTime,
    },
    /// Sent for each feature file that was successfully parsed.
    TestSourceRead {
        time: SystemTime,
    },
    /// Sent for each step that could not be matched to a step definition.
    SnippetsSuggested {
        time: SystemTime,
        uri: String,
        step_locations: Vec<PickleLocation>,
        snippets: Vec<String>,
    },
    /// Sent before starting the execution of a test case.
    TestCaseStarted {
        time: SystemTime,
    },
    /// Sent after the execution of a test case.
    TestStepStarted {
        time: SystemTime,
        test_step: &'a TestStep,
    },
    /// Sent when a hook wants to embed media into a report.
    Embed {
        time: SystemTime,
        data: &'a [u8],
        mime_type: String,
    },
    /// Sent when a hook wants to add some text to a report.
    Write {
        time: SystemTime,
        text: String,
    },
    /// Sent after the execution of a test step.
    TestStepFinished {
        time: SystemTime,
        test_step: &'a TestStep,
        result: &'a TestResult,
    },
    /// Sent after the execution of a test step.
    TestCaseFinished {
        time: SystemTime,
        test_case: &'a TestCase,
        result: TestResult,
    },
    /// Sent after all executions of test steps are finished.
    ///
    /// This is the last event that will be sent.
    TestRunFinished {
        time: SystemTime,
    },
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}

/// A trait to listen to cucumber execution events.
pub trait EventListener: Debug {
    fn on_event(&mut self, event: &Event);
}


#[cfg(test)]
mod tests {
    use super::*;

    fn assert_sync<T: Sync>() {}
    fn assert_send<T: Send>() {}

    #[test]
    fn test_send_sync() {
        assert_send::<Event>();
        assert_sync::<Event>();
    }
}