use std::time::SystemTime;

use gherkin::cuke::{Cuke, Tag};

use api::{self, TestResult, TestResultStatus};
use api::event::Event;
use runner::{EventPublisher, CukeStepTestStep, HookTestStep};
use runtime;

#[derive(Debug)]
pub struct TestCase<'c> {
    pub uri: &'c str,
    pub cuke: &'c Cuke<'c>,
    pub test_steps: Vec<CukeStepTestStep<'c>>,
    pub before_hooks: Vec<HookTestStep<'c>>,
    pub after_hooks: Vec<HookTestStep<'c>>,
    pub dry_run: bool,
}

impl<'s> api::TestCase for TestCase<'s> {
    fn get_test_steps(&self) -> Vec<api::TestStep> {
        let mut test_steps = Vec::new();

        for before_hook in &self.before_hooks {
            test_steps.push(api::TestStep::Hook(before_hook as &dyn api::HookTestStep));
        }

        for test_step in &self.test_steps {
            for before_step_hook_step in &test_step.before_step_hook_steps {
                test_steps.push(api::TestStep::Hook(before_step_hook_step as &dyn api::HookTestStep));
            }

            test_steps.push(api::TestStep::Cuke(test_step as &dyn api::CukeStepTestStep));

            for after_step_hook_step in &test_step.after_step_hook_steps {
                test_steps.push(api::TestStep::Hook(after_step_hook_step as &dyn api::HookTestStep));
            }
        }

        for after_hook in &self.after_hooks {
            test_steps.push(api::TestStep::Hook(after_hook as &dyn api::HookTestStep));
        }

        test_steps
    }

    fn get_name(&self) -> &str {
        &self.cuke.name
    }

    fn get_scenario_designation(&self) -> String {
        format!("{}:{} # {}", self.get_uri(), self.get_line(), self.get_name())
    }

    fn get_uri(&self) -> &str {
        &self.uri
    }

    fn get_line(&self) -> u32 {
        self.cuke.locations[0].line
    }

    fn get_tags(&self) -> &[Tag] {
        &self.cuke.tags
    }
}

pub fn run<EP: EventPublisher>(test_case: TestCase, event_publisher: &EP) {
    let start_time = SystemTime::now();
    event_publisher.send(Event::TestCaseStarted {
        time: start_time,
        uri: test_case.uri,
        feature: test_case.cuke.feature,
        background: test_case.cuke.background,
        scenario_definition: &test_case.cuke.scenario_definition,
        test_case: &test_case as &dyn api::TestCase,
    });

    let mut skip_next_step = test_case.dry_run;
    let mut scenario = runtime::Scenario::new(test_case.uri, &test_case.cuke, event_publisher);

    for before_hook in &test_case.before_hooks {
        let hook_result = before_hook.run(event_publisher, &test_case, &mut scenario, test_case.dry_run);
        skip_next_step = skip_next_step || !hook_result.status.eq(&TestResultStatus::Passed);
        scenario.add_test_result(hook_result);
    }

    for step in &test_case.test_steps {
        let step_result = step.run(event_publisher, &test_case, &mut scenario, skip_next_step);
        skip_next_step = skip_next_step || !step_result.status.eq(&TestResultStatus::Passed);
        scenario.add_test_result(step_result);
    }

    for after_hook in &test_case.after_hooks {
        let hook_result = after_hook.run(event_publisher, &test_case, &mut scenario, test_case.dry_run);
        scenario.add_test_result(hook_result);
    }

    let stop_time = SystemTime::now();
    let duration = match stop_time.duration_since(start_time) {
        Ok(duration) => duration,
        Err(system_time_error) => system_time_error.duration(),
    };
    let test_result = TestResult {
        status: scenario.get_status(),
        duration: Some(duration),
        error: scenario.into_error(),
    };
    event_publisher.send(Event::TestCaseFinished {
        time: stop_time,
        uri: test_case.uri,
        feature: test_case.cuke.feature,
        background: test_case.cuke.background,
        scenario_definition: &test_case.cuke.scenario_definition,
        result: &test_result,
        test_case: &test_case as &dyn api::TestCase,
    });
}
