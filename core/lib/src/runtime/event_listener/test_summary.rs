use std::io::Write;
use std::time::Instant;
use std::sync::Mutex;
use std::cell::RefCell;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use api::event::{Event, EventListener};
use api::TestResultStatus;

#[derive(Debug, Default)]
struct StatusSummary {
    total: u32,
    passed: u32,
    skipped: u32,
    pending: u32,
    undefined: u32,
    ambiguous: u32,
    failed: u32,
}

impl StatusSummary {
    fn add_status(&mut self, status: TestResultStatus) {
        self.total += 1;

        match status {
            TestResultStatus::Passed => self.passed += 1,
            TestResultStatus::Skipped => self.skipped += 1,
            TestResultStatus::Pending => self.pending += 1,
            TestResultStatus::Undefined => self.undefined += 1,
            TestResultStatus::Ambiguous => self.ambiguous += 1,
            TestResultStatus::Failed => self.failed += 1,
        }
    }
}

#[derive(Debug)]
pub struct TestSummaryListener {
    status_summary: RefCell<StatusSummary>,
    start_time: Instant,
}

impl TestSummaryListener {
    pub fn new() -> TestSummaryListener {
        TestSummaryListener {
            status_summary: Default::default(),
            start_time: Instant::now(),
        }
    }

    pub fn print_test_summary(&self) {
        let summary = self.status_summary.borrow();
        print_test_summary(&summary, self.start_time);
    }
}

impl EventListener for TestSummaryListener {
    fn on_event(&self, event: &Event) {
        if let Event::TestCaseFinished { ref result, .. } = *event {
            self.status_summary.borrow_mut().add_status(result.status)
        }
    }
}

#[derive(Debug)]
pub struct SyncTestSummaryListener {
    status_summary: Mutex<RefCell<StatusSummary>>,
    start_time: Instant,
}

impl SyncTestSummaryListener {
    pub fn new() -> SyncTestSummaryListener {
        SyncTestSummaryListener {
            status_summary: Default::default(),
            start_time: Instant::now(),
        }
    }

    pub fn print_test_summary(&self) {
        let status_summary_lock = self.status_summary.lock().unwrap();
        let summary = status_summary_lock.borrow();
        print_test_summary(&summary, self.start_time);
    }
}

impl EventListener for SyncTestSummaryListener {
    fn on_event(&self, event: &Event) {
        if let Event::TestCaseFinished { ref result, .. } = *event {
            self.status_summary.lock().unwrap().borrow_mut().add_status(result.status)
        }
    }
}

fn print_test_summary(summary: &StatusSummary, start_time: Instant) {
    let time_elapsed = start_time.elapsed();

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    writeln!(&mut stdout, "Ran {} tests in {:?}", summary.total, time_elapsed).unwrap();
    write_conditional_colored(&mut stdout, || summary.passed > 0, Color::Green,
        format!("    Passed: {}", summary.passed));
    write_conditional_colored(&mut stdout, || summary.skipped > 0, Color::Yellow,
        format!("    Skipped: {}", summary.skipped));
    write_conditional_colored(&mut stdout, || summary.pending > 0, Color::Yellow,
        format!("    Pending: {}", summary.pending));
    write_conditional_colored(&mut stdout, || summary.undefined > 0, Color::Red,
        format!("    Undefined: {}", summary.undefined));
    write_conditional_colored(&mut stdout, || summary.ambiguous > 0, Color::Red,
        format!("    Ambiguous: {}", summary.ambiguous));
    write_conditional_colored(&mut stdout, || summary.failed > 0, Color::Red,
        format!("    Failed: {}", summary.failed));
    writeln!(&mut stdout).unwrap();
}

fn write_conditional_colored<C: Fn() -> bool>(stdout: &mut StandardStream,
    condition: C, color: Color, text: String) {

    let condition_result = condition();

    if condition_result {
        stdout.set_color(ColorSpec::new().set_fg(Some(color))).unwrap();
    }
    stdout.write_all(text.as_bytes()).unwrap();
    stdout.write_all(b"\n").unwrap();
    if condition_result {
        stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
    }
}
