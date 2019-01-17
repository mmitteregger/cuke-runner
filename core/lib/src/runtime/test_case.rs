use std::time::SystemTime;

use gherkin::event::PickleEvent;
use gherkin::pickle::PickleTag;

use api::{self, FeatureFile, TestResult, TestResultStatus};
use api::event::Event;
use runner::{EventBus, PickleStepTestStep, HookTestStep};
use runtime;

#[derive(Debug)]
pub struct TestCase<'c> {
    pub feature_file: &'c FeatureFile,
    pub pickle_event: &'c PickleEvent,
    pub test_steps: Vec<PickleStepTestStep<'c>>,
    pub before_hooks: Vec<HookTestStep<'c>>,
    pub after_hooks: Vec<HookTestStep<'c>>,
    pub dry_run: bool,
}

impl<'s> api::TestCase for TestCase<'s> {
    fn get_test_steps(&self) -> Vec<api::TestStep> {
        let mut test_steps = Vec::new();

        for before_hook in &self.before_hooks {
            test_steps.push(api::TestStep::Hook(before_hook as &api::HookTestStep));
        }

        for test_step in &self.test_steps {
            for before_step_hook_step in &test_step.before_step_hook_steps {
                test_steps.push(api::TestStep::Hook(before_step_hook_step as &api::HookTestStep));
            }

            test_steps.push(api::TestStep::Pickle(test_step as &api::PickleStepTestStep));

            for after_step_hook_step in &test_step.after_step_hook_steps {
                test_steps.push(api::TestStep::Hook(after_step_hook_step as &api::HookTestStep));
            }
        }

        for after_hook in &self.after_hooks {
            test_steps.push(api::TestStep::Hook(after_hook as &api::HookTestStep));
        }

        test_steps
    }

    fn get_name(&self) -> &String {
        &self.pickle_event.pickle.name
    }

    fn get_scenario_designation(&self) -> String {
        format!("{}:{} # {}", self.get_uri(), self.get_line(), self.get_name())
    }

    fn get_uri(&self) -> &String {
        &self.pickle_event.uri
    }

    fn get_line(&self) -> u32 {
        self.pickle_event.pickle.locations[0].line
    }

    fn get_tags(&self) -> &Vec<PickleTag> {
        &self.pickle_event.pickle.tags
    }
}

pub fn run(test_case: TestCase, event_bus: &EventBus) {
    let feature_file = &test_case.feature_file;

    let start_time = SystemTime::now();
    event_bus.send(Event::TestCaseStarted {
        time: start_time,
        feature_file,
        test_case: &test_case as &api::TestCase,
    });

    let mut skip_next_step = test_case.dry_run;
    let mut scenario = runtime::Scenario::new(&test_case.pickle_event, event_bus);

    for before_hook in &test_case.before_hooks {
        let hook_result = before_hook.run(event_bus, feature_file, &test_case, &mut scenario, test_case.dry_run);
        skip_next_step = skip_next_step || !hook_result.status.eq(&TestResultStatus::Passed);
        scenario.add_test_result(hook_result);
    }

    for step in &test_case.test_steps {
        let step_result = step.run(event_bus, feature_file, &test_case, &mut scenario, skip_next_step);
        skip_next_step = skip_next_step || !step_result.status.eq(&TestResultStatus::Passed);
        scenario.add_test_result(step_result);
    }

    for after_hook in &test_case.after_hooks {
        let hook_result = after_hook.run(event_bus, feature_file, &test_case, &mut scenario, test_case.dry_run);
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
    event_bus.send(Event::TestCaseFinished {
        time: stop_time,
        feature_file,
        result: &test_result,
        test_case: &test_case as &api::TestCase,
    });
}
