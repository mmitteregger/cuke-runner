use std::time::SystemTime;

use gherkin::event::PickleEvent;
use gherkin::pickle::{PickleTag};

use error::Error;
use runner::EventBus;
use api::{self, TestResult, TestResultStatus};
use api::event::Event;
use glue;

#[derive(Debug)]
pub struct Scenario<'a, 'b, 'c: 'b> {
    test_results: Vec<TestResult>,
    tags: &'a Vec<PickleTag>,
    uri: &'a String,
    name: &'a String,
    id: String,
    lines: Vec<u32>,
    event_bus: &'b EventBus<'c>,
    pub glue_scenario: glue::Scenario,
}

impl<'a, 'b, 'c> Scenario<'a, 'b, 'c> {
    pub fn new(pickle_event: &'a PickleEvent, event_bus: &'b EventBus<'c>) -> Scenario<'a, 'b, 'c> {
        let pickle = &pickle_event.pickle;

        let test_results = Vec::new();
        let tags = &pickle.tags;
        let uri = &pickle_event.uri;
        let name = &pickle.name;
        let locations = &pickle.locations;
        let id = pickle_event.uri.clone() + ":" + &locations[0].line.to_string();
        let lines = locations.iter()
            .map(|location| location.line)
            .collect::<Vec<u32>>();

        Scenario {
            test_results,
            tags,
            uri,
            name,
            id,
            lines,
            event_bus,
            glue_scenario: glue::Scenario::new(),
        }
    }

    pub fn add_test_result(&mut self, test_result: TestResult) {
        self.test_results.push(test_result);
    }

    pub fn into_error(self) -> Option<Error> {
        if self.test_results.is_empty() {
            None
        } else {
            self.test_results.into_iter()
                .max_by_key(TestResult::get_status)
                .map(|test_result| test_result.error)
                .unwrap_or(None)
        }
    }
}

impl<'a, 'b, 'c> api::Scenario for Scenario<'a, 'b, 'c> {
    fn get_status(&self) -> TestResultStatus {
        self.test_results.iter()
            .map(TestResult::get_status)
            .max()
            .unwrap_or(TestResultStatus::Undefined)
    }

    fn embed(&self, data: &[u8], mime_type: String) {
        self.event_bus.send(Event::Embed {
            time: SystemTime::now(),
            data,
            mime_type,
        });
    }

    fn write(&self, text: String) {
        self.event_bus.send(Event::Write {
            time: SystemTime::now(),
            text,
        });
    }

    fn get_name(&self) -> &String {
        self.name
    }

    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_uri(&self) -> &String {
        self.uri
    }

    fn get_lines(&self) -> &Vec<u32> {
        &self.lines
    }
}
