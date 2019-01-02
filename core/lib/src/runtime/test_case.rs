use std::time::SystemTime;
use std::rc::Rc;

use gherkin::event::PickleEvent;
use gherkin::pickle::PickleTag;

use api;
use api::event::Event;
use api::{TestResult, TestResultStatus, Scenario};
use runner::{EventBus, PickleStepTestStep, HookTestStep};
use runtime;

#[derive(Debug)]
pub struct TestCase {
    pub test_steps: Vec<PickleStepTestStep>,
    pub before_hooks: Vec<HookTestStep>,
    pub after_hooks: Vec<HookTestStep>,
    pub pickle_event: PickleEvent,
    pub dry_run: bool,
}

trait AsApiTestCase {
    fn as_api_test_case(self: Rc<Self>) -> Rc<api::TestCase>;
}

impl AsApiTestCase for TestCase {
    fn as_api_test_case(self: Rc<Self>) -> Rc<api::TestCase> {
        self
    }
}

impl api::TestCase for TestCase {
    fn get_test_steps(&self) -> Vec<Box<&api::TestStep>> {
//        unimplemented!("TestCase::get_test_steps(&self) -> &Vec<Box<api::TestStep>>")
        let mut test_steps = Vec::new();

        for before_hook in &self.before_hooks {
            test_steps.push(Box::new(before_hook as &api::TestStep));
        }

        for test_step in &self.test_steps {
            for before_step_hook_step in &test_step.before_step_hook_steps {
                test_steps.push(Box::new(before_step_hook_step as &api::TestStep));
            }

            test_steps.push(Box::new(test_step as &api::TestStep));

            for after_step_hook_step in &test_step.after_step_hook_steps {
                test_steps.push(Box::new(after_step_hook_step as &api::TestStep));
            }
        }

        for after_hook in &self.after_hooks {
            test_steps.push(Box::new(after_hook as &api::TestStep));
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

pub fn run(test_case: Rc<TestCase>, event_bus: &EventBus) {
    let start_time = SystemTime::now();
    event_bus.send(Event::TestCaseStarted {
        time: start_time,
        test_case: &test_case.clone().as_api_test_case(),
    });

    let mut skip_next_step = test_case.dry_run;
    let mut scenario = runtime::Scenario::new(&test_case.pickle_event, event_bus);
    let language = &test_case.pickle_event.pickle.language;

    for before_hook in &test_case.before_hooks {
        let hook_result = before_hook.run(event_bus, language, &mut scenario, test_case.dry_run);
        skip_next_step = skip_next_step || !hook_result.status.eq(&TestResultStatus::Passed);
        scenario.add_test_result(hook_result);
    }

    for step in &test_case.test_steps {
        let step_result = step.run(event_bus, language, &mut scenario, skip_next_step);
        skip_next_step = skip_next_step || !step_result.status.eq(&TestResultStatus::Passed);
        scenario.add_test_result(step_result);
    }

    for after_hook in &test_case.after_hooks {
        let hook_result = after_hook.run(event_bus, language, &mut scenario, test_case.dry_run);
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
        result: &test_result,
        test_case: &test_case.as_api_test_case(),
    });
}
