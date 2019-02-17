use std::time::SystemTime;

use gherkin::cuke::{Cuke, Tag};

use error::Error;
use runner::EventBus;
use api::{TestResult, TestResultStatus};
use api::event::Event;
use glue;

#[derive(Debug)]
pub struct Scenario<'a, 'b, 'c: 'b> {
    test_results: Vec<TestResult>,
    tags: &'a Vec<Tag<'a>>,
    uri: &'a str,
    name: &'a str,
    id: String,
    lines: Vec<u32>,
    event_bus: &'b EventBus<'c>,
    pub(crate) glue_scenario: glue::scenario::Scenario,
}

impl<'a, 'b, 'c> Scenario<'a, 'b, 'c> {
    pub fn new(uri: &'a str, cuke: &'a Cuke, event_bus: &'b EventBus<'c>) -> Scenario<'a, 'b, 'c> {
        let test_results = Vec::new();
        let tags = &cuke.tags;
        let name = &cuke.name;
        let locations = &cuke.locations;
        let id = uri.to_string() + ":" + &locations[0].line.to_string();
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
            glue_scenario: glue::scenario::Scenario::new(),
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

    pub fn get_status(&self) -> TestResultStatus {
        self.test_results.iter()
            .map(TestResult::get_status)
            .max()
            .unwrap_or(TestResultStatus::Undefined)
    }

    pub fn embed(&self, data: &[u8], mime_type: String) {
        self.event_bus.send(Event::Embed {
            time: SystemTime::now(),
            data,
            mime_type,
        });
    }

    pub fn write(&self, text: &str) {
        self.event_bus.send(Event::Write {
            time: SystemTime::now(),
            text,
        });
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_uri(&self) -> &str {
        self.uri
    }

    pub fn get_lines(&self) -> &[u32] {
        &self.lines
    }
}
