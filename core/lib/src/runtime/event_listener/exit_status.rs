use api::event::{Event, EventListener};
use api::TestResultStatus;

#[derive(Debug, Default)]
pub struct ExitStatusListener {
    test_result_statuses: Vec<TestResultStatus>,
}

impl ExitStatusListener {
    pub fn new() -> ExitStatusListener {
        ExitStatusListener::default()
    }

    pub fn get_exit_status(&self, strict: bool) -> i32 {
        self.test_result_statuses.iter()
            .max()
            .map(|status| if status.is_ok(strict) { 0 } else { 1 })
            .unwrap_or(0)
    }
}

impl EventListener for ExitStatusListener {
    fn on_event(&mut self, event: &Event) {
        if let Event::TestCaseFinished { ref result, .. } = *event {
            self.test_result_statuses.push(result.status);
        }
    }
}
