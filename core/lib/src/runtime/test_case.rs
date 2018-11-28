use std::time::SystemTime;

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

impl TestCase {
    pub fn run(&self, event_bus: &EventBus) {
        let start_time = SystemTime::now();
        event_bus.send(Event::TestCaseStarted {
            time: start_time,
        });

        let mut skip_next_step = self.dry_run;
        let mut scenario = runtime::Scenario::new(&self.pickle_event, event_bus);
        let language = &self.pickle_event.pickle.language;

        for before_hook in &self.before_hooks {
            let hook_result = before_hook.run(event_bus, language, &mut scenario, self.dry_run);
            skip_next_step = skip_next_step || !hook_result.status.eq(&TestResultStatus::Passed);
            scenario.add_test_result(hook_result);
        }

        for step in &self.test_steps {
            let step_result = step.run(event_bus, language, &mut scenario, skip_next_step);
            skip_next_step = skip_next_step || !step_result.status.eq(&TestResultStatus::Passed);
            scenario.add_test_result(step_result);
        }

        for after_hook in &self.after_hooks {
            let hook_result = after_hook.run(event_bus, language, &mut scenario, self.dry_run);
            scenario.add_test_result(hook_result);
        }

        let stop_time = SystemTime::now();
        let duration = match stop_time.duration_since(start_time) {
            Ok(duration) => duration,
            Err(system_time_error) => system_time_error.duration(),
        };
        event_bus.send(Event::TestCaseFinished {
            time: stop_time,
            result: TestResult {
                status: scenario.get_status(),
                duration: Some(duration),
                error: scenario.into_error(),
            },
            test_case: self as &api::TestCase,
        });
    }
}

impl api::TestCase for TestCase {
    fn get_test_steps(&self) -> &Vec<Box<api::TestStep>> {
        unimplemented!()
    }

    fn get_name(&self) -> &String {
        unimplemented!()
    }

    fn get_scenario_designation(&self) -> &String {
        unimplemented!()
    }

    fn get_uri(&self) -> &String {
        unimplemented!()
    }

    fn get_line(&self) -> usize {
        unimplemented!()
    }

    fn get_tags(&self) -> &Vec<PickleTag> {
        unimplemented!()
    }

//    @Override
//    public List<TestStep> getTestSteps() {
//        List<TestStep> testSteps = new ArrayList<TestStep>();
//        testSteps.addAll(beforeHooks);
//        for (PickleStepTestStep step : this.testSteps) {
//            testSteps.addAll(step.getBeforeStepHookSteps());
//            testSteps.add(step);
//            testSteps.addAll(step.getAfterStepHookSteps());
//        }
//        testSteps.addAll(afterHooks);
//        return testSteps;
//    }
//
//    @Override
//    public String getName() {
//        return pickleEvent.pickle.getName();
//    }
//
//    @Override
//    public String getScenarioDesignation() {
//        return fileColonLine(pickleEvent.pickle.getLocations().get(0)) + " # " + getName();
//    }
//
//    @Override
//    public String getUri() {
//        return pickleEvent.uri;
//    }
//
//    @Override
//    public int getLine() {
//        return pickleEvent.pickle.getLocations().get(0).getLine();
//    }
//
//    private String fileColonLine(PickleLocation location) {
//        return pickleEvent.uri + ":" + location.getLine();
//    }
//
//    @Override
//    public List<PickleTag> getTags() {
//        return pickleEvent.pickle.getTags();
//    }
}
