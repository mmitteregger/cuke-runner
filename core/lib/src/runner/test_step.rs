use std::time::{SystemTime, Duration};

use gherkin::pickle::PickleStep;

use error::{Result, Error};
use api::{self, event::Event, HookType, SourceCodeLocation, TestResult, TestResultStatus};
use runner::EventBus;
use runtime::{StepDefinitionMatch, HookDefinitionMatch, Scenario};

#[derive(Debug)]
pub struct HookTestStep {
    pub definition_match: HookDefinitionMatch,
    pub hook_type: HookType,
}

impl HookTestStep {
    pub fn run(
        &self,
        event_bus: &EventBus,
        language: &str,
        scenario: &mut Scenario,
        skip: bool,
    ) -> TestResult
    {
        let test_step = self as &api::TestStep;
        run_test_step(test_step, &self.definition_match, event_bus, language, scenario, skip)
    }
}

impl api::HookTestStep for HookTestStep {
    fn get_hook_type(&self) -> HookType {
        self.hook_type
    }
}

impl api::TestStep for HookTestStep {
    fn get_location(&self) -> &SourceCodeLocation {
        self.definition_match.get_location()
    }
}

#[derive(Debug)]
pub struct PickleStepTestStep {
    pub uri: String,
    pub before_step_hook_steps: Vec<HookTestStep>,
    pub after_step_hook_steps: Vec<HookTestStep>,
    pub step_definition_match: Box<StepDefinitionMatch>,
}

impl PickleStepTestStep {
    pub fn run(
        &self,
        event_bus: &EventBus,
        language: &str,
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
            let hook_result = before_step_hook_step.run(event_bus, language, scenario, skip);
            skip_self = skip_self || !hook_result.status.eq(&TestResultStatus::Passed);
            results.push(hook_result);
        }

        let test_step = self as &api::TestStep;
        let self_result = run_test_step(test_step, &*self.step_definition_match,
                event_bus, language, scenario, skip_self);
        results.push(self_result);

        for after_step_hook_step in &self.after_step_hook_steps {
            let hook_result = after_step_hook_step.run(event_bus, language, scenario, skip);
            results.push(hook_result);
        }

        results.into_iter()
            .max_by_key(TestResult::get_status)
            .expect("at least one test result")
    }
}

impl api::PickleStepTestStep for PickleStepTestStep {
    fn get_pattern(&self) -> Option<&String> {
        self.step_definition_match.get_pattern()
    }

    fn get_pickle_step(&self) -> &PickleStep {
        &self.step_definition_match.get_step()
    }

    fn get_definition_argument<A: api::Argument + Sized>(&self) -> &Vec<A> {
        unimplemented!();
//        DefinitionArgument::create_arguments(self.step_definition_match.get_arguments())
    }

    fn get_step_argument(&self) -> &Vec<Box<::gherkin::pickle::Argument>> {
        &self.step_definition_match.get_step().arguments
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

impl api::TestStep for PickleStepTestStep {
    fn get_location(&self) -> &SourceCodeLocation {
        self.step_definition_match.get_location()
    }
}

fn run_test_step(
    test_step: &api::TestStep,
    definition_match: &StepDefinitionMatch,
    event_bus: &EventBus,
    language: &str,
    scenario: &mut Scenario,
    skip: bool,
) -> TestResult
{
    let start_time = SystemTime::now();
    event_bus.send(Event::TestStepStarted {
        time: start_time,
        test_step,
    });

    let step_result = execute_step(definition_match, language, scenario, skip);
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
        test_step,
        result: &result,
    });
    result
}

fn execute_step(
    definition_match: &StepDefinitionMatch,
    language: &str,
    scenario: &mut Scenario,
    skip: bool
) -> Result<TestResultStatus>
{
    let test_result_type = if skip {
        definition_match.dry_run_step(language, scenario)?;
        TestResultStatus::Skipped
    } else {
        definition_match.run_step(language, scenario)?;
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
