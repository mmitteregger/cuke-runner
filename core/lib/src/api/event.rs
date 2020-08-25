use std::fmt::Debug;
use std::time::SystemTime;

use gherkin::ast::{Feature, Background, Scenario};
use gherkin::cuke::Cuke;

use crate::api::{TestCase, TestStep, TestResult};

/// An execution event that is generated by cucumber.
#[derive(Debug)]
pub enum Event<'e, 's> {
    /// Sent for each feature file that was successfully parsed.
    ///
    /// These events are currently first ones being sent, but this may be changed in the future.
    TestSourceRead {
        time: SystemTime,
        uri: &'e str,
        source: &'e str,
        feature: &'e Feature,
        cukes: &'e [Cuke<'e>],
    },
    /// Sent before any execution of test steps is started,
    /// but after all feature files have been parsed.
    TestRunStarted {
        time: SystemTime,
        num_cukes: usize,
    },
    /// Sent before starting the execution of a test case.
    TestCaseStarted {
        time: SystemTime,
        uri: &'e str,
        feature: &'e Feature,
        feature_background: Option<&'e Background>,
        rule_background: Option<&'e Background>,
        scenario: &'e Scenario,
        test_case: &'e dyn TestCase,
    },
    /// Sent after the execution of a test case.
    TestStepStarted {
        time: SystemTime,
        uri: &'e str,
        feature: &'e Feature,
        feature_background: Option<&'e Background>,
        rule_background: Option<&'e Background>,
        scenario: &'e Scenario,
        test_case: &'e dyn TestCase,
        test_step: &'e TestStep<'e, 's>,
    },
    /// Sent when a hook wants to embed media into a report.
    Embed {
        time: SystemTime,
        data: &'e [u8],
        mime_type: String,
    },
    /// Sent when a hook wants to add some text to a report.
    Write {
        time: SystemTime,
        text: &'e str,
    },
    /// Sent after the execution of a test step.
    TestStepFinished {
        time: SystemTime,
        uri: &'e str,
        feature: &'e Feature,
        feature_background: Option<&'e Background>,
        rule_background: Option<&'e Background>,
        scenario: &'e Scenario,
        test_case: &'e dyn TestCase,
        test_step: &'e TestStep<'e, 's>,
        result: &'e TestResult,
    },
    /// Sent after the execution of a test step.
    TestCaseFinished {
        time: SystemTime,
        uri: &'e str,
        feature: &'e Feature,
        feature_background: Option<&'e Background>,
        rule_background: Option<&'e Background>,
        scenario: &'e Scenario,
        test_case: &'e dyn TestCase,
        result: &'e TestResult,
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
pub trait EventListener: Debug + Send {
    fn on_event(&self, event: &Event<'_, '_>);
}

/// A trait to listen to cucumber execution events
/// that can be safely shared between multiple threads.
pub trait SyncEventListener: EventListener + Sync {}

impl<T: EventListener + Sync> SyncEventListener for T {}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_sync<T: Sync>() {}
    fn assert_send<T: Send>() {}

    #[test]
    fn test_send_sync() {
        assert_send::<Event<'_, '_>>();
        assert_sync::<Event<'_, '_>>();
    }
}
