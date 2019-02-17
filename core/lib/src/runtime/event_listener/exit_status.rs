use std::sync::Mutex;
use std::cell::RefCell;
use api::event::{Event, EventListener};
use api::TestResultStatus;

#[derive(Debug, Default)]
pub struct ExitStatusListener {
    test_result_statuses: RefCell<Vec<TestResultStatus>>,
}

impl ExitStatusListener {
    pub fn new() -> ExitStatusListener {
        ExitStatusListener::default()
    }

    pub fn get_exit_status(&self, strict: bool) -> i32 {
        let test_result_statuses = self.test_result_statuses.borrow();
        get_exit_status(&test_result_statuses, strict)
    }
}

impl EventListener for ExitStatusListener {
    fn on_event(&self, event: &Event) {
        if let Event::TestCaseFinished { ref result, .. } = *event {
            self.test_result_statuses.borrow_mut().push(result.status);
        }
    }
}

#[derive(Debug, Default)]
pub struct SyncExitStatusListener {
    test_result_statuses: Mutex<RefCell<Vec<TestResultStatus>>>,
}

impl SyncExitStatusListener {
    pub fn new() -> SyncExitStatusListener {
        SyncExitStatusListener::default()
    }

    pub fn get_exit_status(&self, strict: bool) -> i32 {
        let test_result_statuses_lock = self.test_result_statuses.lock().unwrap();
        let test_result_statuses = test_result_statuses_lock.borrow();
        get_exit_status(&test_result_statuses, strict)
    }
}

impl EventListener for SyncExitStatusListener {
    fn on_event(&self, event: &Event) {
        if let Event::TestCaseFinished { ref result, .. } = *event {
            self.test_result_statuses.lock().unwrap().borrow_mut().push(result.status);
        }
    }
}

fn get_exit_status(test_result_statuses: &[TestResultStatus], strict: bool) -> i32 {
    test_result_statuses.iter()
        .max()
        .map(|status| if status.is_ok(strict) { 0 } else { 1 })
        .unwrap_or(0)
}
