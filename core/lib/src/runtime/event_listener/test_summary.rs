use std::time::Instant;
use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use api::TestResultStatus;
use api::event::{Event, EventListener};

#[derive(Debug)]
pub struct TestSummaryListener {
    status_summary: StatusSummary,
    start_time: Instant,
}

impl TestSummaryListener {
    pub fn new() -> TestSummaryListener {
        TestSummaryListener {
            status_summary: StatusSummary::default(),
            start_time: Instant::now(),
        }
    }
}

#[derive(Debug, Default)]
pub struct StatusSummary {
    total: u32,
    passed: u32,
    skipped: u32,
    pending: u32,
    undefined: u32,
    ambiguous: u32,
    failed: u32,
}

impl StatusSummary {
    fn add_status(&mut self, status: &TestResultStatus) {
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

impl EventListener for TestSummaryListener {
    fn on_event(&mut self, event: &Event) {
        match *event {
            Event::TestCaseFinished { ref result, ..} => {
                self.status_summary.add_status(&result.status)
            },
            _ => {},
        }
    }
}

impl Drop for TestSummaryListener {
    fn drop(&mut self) {
        let summary = &self.status_summary;
        let time_elapsed = self.start_time.elapsed();

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
}

fn write_conditional_colored<C: Fn() -> bool>(stdout: &mut StandardStream,
    condition: C, color: Color, text: String) {

    let condition_result = condition();

    if condition_result {
        stdout.set_color(ColorSpec::new().set_fg(Some(color))).unwrap();
    }
    stdout.write(text.as_bytes()).unwrap();
    stdout.write(b"\n").unwrap();
    if condition_result {
        stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
    }
}
