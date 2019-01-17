use std::time::{SystemTime, Duration};

use gherkin::pickle::PickleStep;

use error::{Result, Error};
use api::{self, event::Event, FeatureFile, HookType, CodeLocation, TestCase, TestResult, TestResultStatus};
use glue::step::argument::StepArgument;
use runner::EventBus;
use runtime::{StepDefinitionMatch, Scenario};

#[derive(Debug)]
pub struct HookTestStep<'s> {
    pub definition_match: StepDefinitionMatch<'s>,
    pub hook_type: HookType,
}

impl<'s> HookTestStep<'s> {
    pub fn run(
        &self,
        event_bus: &EventBus,
        feature_file: &FeatureFile,
        test_case: &TestCase,
        scenario: &mut Scenario,
        skip: bool,
    ) -> TestResult
    {
        let test_step = &api::TestStep::Hook(self as &api::HookTestStep);
        run_test_step(feature_file, test_case, test_step, &self.definition_match, event_bus, scenario, skip)
    }
}

impl<'s> api::HookTestStep<'s> for HookTestStep<'s> {
    fn get_code_location(&self) -> Option<&CodeLocation> {
        self.definition_match.get_location()
    }

    fn get_hook_type(&self) -> HookType {
        self.hook_type
    }
}

#[derive(Debug)]
pub struct PickleStepTestStep<'s> {
    pub uri: String,
    pub before_step_hook_steps: Vec<HookTestStep<'s>>,
    pub after_step_hook_steps: Vec<HookTestStep<'s>>,
    pub step_definition_match: StepDefinitionMatch<'s>,
}

impl<'s> PickleStepTestStep<'s> {
    pub fn run(
        &self,
        event_bus: &EventBus,
        feature_file: &FeatureFile,
        test_case: &TestCase,
        scenario: &mut Scenario,
        skip: bool,
    ) -> TestResult
    {
        let mut skip_self = skip;
        let mut results = Vec::with_capacity(
            self.before_step_hook_steps.len()
                + 1 // self
                + self.after_step_hook_steps.len()
        );

        for before_step_hook_step in &self.before_step_hook_steps {
            let hook_result = before_step_hook_step.run(event_bus, feature_file, test_case, scenario, skip);
            skip_self = skip_self || !hook_result.status.eq(&TestResultStatus::Passed);
            results.push(hook_result);
        }

        let test_step = &api::TestStep::Pickle(self as &api::PickleStepTestStep);
        let self_result = run_test_step(feature_file, test_case, test_step, &self.step_definition_match,
                event_bus, scenario, skip_self);
        results.push(self_result);

        for after_step_hook_step in &self.after_step_hook_steps {
            let hook_result = after_step_hook_step.run(event_bus, feature_file, test_case, scenario, skip);
            results.push(hook_result);
        }

        results.into_iter()
            .max_by_key(TestResult::get_status)
            .expect("at least one test result")
    }
}

impl<'s> api::PickleStepTestStep<'s> for PickleStepTestStep<'s> {
    fn get_code_location(&self) -> Option<&CodeLocation> {
        self.step_definition_match.get_location()
    }

    fn get_pattern(&self) -> Option<&String> {
        self.step_definition_match.get_pattern()
    }

    fn get_pickle_step(&self) -> &PickleStep {
        &self.step_definition_match.get_step()
    }

    fn get_arguments(&self) -> &[StepArgument] {
        &self.step_definition_match.get_arguments()
    }

    fn get_step_line(&self) -> u32 {
        self.step_definition_match.get_step().locations.last()
            .map(|location| location.line)
            .unwrap_or(0)
    }

    fn get_step_location(&self) -> String {
        format!("{}:{}", &self.uri, &self.get_step_line())
    }

    fn get_step_text(&self) -> &String {
        &self.step_definition_match.get_step().text
    }
}

fn run_test_step(
    feature_file: &FeatureFile,
    test_case: &api::TestCase,
    test_step: &api::TestStep,
    definition_match: &StepDefinitionMatch,
    event_bus: &EventBus,
    scenario: &mut Scenario,
    skip: bool,
) -> TestResult
{
    let start_time = SystemTime::now();
    event_bus.send(Event::TestStepStarted {
        time: start_time,
        feature_file,
        test_case,
        test_step,
    });

    let step_result = execute_step(definition_match, scenario, skip);
    let (status, error) = match step_result {
        Ok(test_result_type) => (test_result_type, None),
        Err(error) => (map_error_to_status(&error), Some(error)),
    };

    let stop_time = SystemTime::now();
    let duration = match stop_time.duration_since(start_time) {
        Ok(duration) => duration,
        Err(system_time_error) => system_time_error.duration(),
    };
    let result = map_status_to_result(status, error, duration);
    event_bus.send(Event::TestStepFinished {
        time: stop_time,
        feature_file,
        test_case,
        test_step,
        result: &result,
    });
    result
}

fn execute_step(
    definition_match: &StepDefinitionMatch,
    scenario: &mut Scenario,
    skip: bool
) -> Result<TestResultStatus>
{
    let test_result_type = if skip {
        definition_match.dry_run_step(scenario)?;
        TestResultStatus::Skipped
    } else {
        definition_match.run_step(scenario)?;
        TestResultStatus::Passed
    };

    Ok(test_result_type)
}

fn map_error_to_status(error: &Error) -> TestResultStatus {
    match error {
        Error::AmbiguousStepDefinitions => TestResultStatus::Ambiguous,
        Error::UndefinedStepDefinition => TestResultStatus::Undefined,
        Error::Pending => TestResultStatus::Pending,
        _ => TestResultStatus::Failed,
    }
}

fn map_status_to_result(status: TestResultStatus, error: Option<Error>, duration: Duration)
                        -> TestResult
{
    if status == TestResultStatus::Skipped && error.is_none() {
        TestResult {
            status: TestResultStatus::Skipped,
            duration: None,
            error: None,
        }
    } else if status == TestResultStatus::Undefined {
        TestResult {
            status: TestResultStatus::Undefined,
            duration: None,
            error: None,
        }
    } else {
        TestResult {
            status,
            duration: Some(duration),
            error,
        }
    }
}
