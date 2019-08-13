use std::cell::RefCell;
use std::io::Write;
use std::sync::Mutex;
use std::time::Instant;

use gherkin::cuke::ScenarioDefinition;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use api::{TestCase, TestResultStatus};
use api::event::{Event, EventListener};

#[derive(Debug, Default)]
struct TestSummary {
    passed: usize,
    skipped: Vec<TestInfo>,
    pending: Vec<TestInfo>,
    undefined: Vec<TestInfo>,
    ambiguous: Vec<TestInfo>,
    failed: Vec<TestInfo>,
}

impl TestSummary {
    fn add_result(&mut self, scenario_definition: &ScenarioDefinition, test_case: &dyn TestCase,
        status: TestResultStatus)
    {
        match status {
            TestResultStatus::Passed => self.passed += 1,
            TestResultStatus::Skipped => self.skipped.push(TestInfo::from((scenario_definition, test_case))),
            TestResultStatus::Pending => self.pending.push(TestInfo::from((scenario_definition, test_case))),
            TestResultStatus::Undefined => self.undefined.push(TestInfo::from((scenario_definition, test_case))),
            TestResultStatus::Ambiguous => self.ambiguous.push(TestInfo::from((scenario_definition, test_case))),
            TestResultStatus::Failed => self.failed.push(TestInfo::from((scenario_definition, test_case))),
        }
    }
}

#[derive(Debug)]
struct TestInfo {
    keyword: String,
    name: String,
    uri: String,
    line: u32,
}

impl<'a> From<(&'a ScenarioDefinition<'a>, &'a dyn TestCase)> for TestInfo {
    fn from((scenario_definition, test_case): (&'a ScenarioDefinition, &'a dyn TestCase)) -> Self {
        TestInfo {
            keyword: scenario_definition.get_keyword().to_owned(),
            name: test_case.get_name().to_owned(),
            uri: test_case.get_uri().to_owned(),
            line: test_case.get_line(),
        }
    }
}

#[derive(Debug)]
pub struct TestSummaryListener {
    test_summary: RefCell<TestSummary>,
    start_time: Instant,
}

impl TestSummaryListener {
    pub fn new() -> TestSummaryListener {
        TestSummaryListener {
            test_summary: Default::default(),
            start_time: Instant::now(),
        }
    }

    pub fn print_test_summary(&self) {
        let summary = self.test_summary.borrow();
        print_test_summary(&summary, self.start_time);
    }
}

impl EventListener for TestSummaryListener {
    fn on_event(&self, event: &Event) {
        if let Event::TestCaseFinished { scenario_definition, test_case, ref result, .. } = *event {
            self.test_summary.borrow_mut()
                .add_result(scenario_definition, test_case, result.status)
        }
    }
}

#[derive(Debug)]
pub struct SyncTestSummaryListener {
    test_summary: Mutex<RefCell<TestSummary>>,
    start_time: Instant,
}

impl SyncTestSummaryListener {
    pub fn new() -> SyncTestSummaryListener {
        SyncTestSummaryListener {
            test_summary: Default::default(),
            start_time: Instant::now(),
        }
    }

    pub fn print_test_summary(&self) {
        let status_summary_lock = self.test_summary.lock().unwrap();
        let summary = status_summary_lock.borrow();
        print_test_summary(&summary, self.start_time);
    }
}

impl EventListener for SyncTestSummaryListener {
    fn on_event(&self, event: &Event) {
        if let Event::TestCaseFinished { scenario_definition, test_case, ref result, .. } = *event {
            self.test_summary.lock().unwrap().borrow_mut()
                .add_result(scenario_definition, test_case, result.status)
        }
    }
}

fn print_test_summary(summary: &TestSummary, start_time: Instant) {
    let time_elapsed = start_time.elapsed();

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    let tests_count = summary.passed
        + summary.skipped.len()
        + summary.pending.len()
        + summary.undefined.len()
        + summary.ambiguous.len()
        + summary.failed.len();

    writeln!(&mut stdout, "Ran {} tests in {:?}", tests_count, time_elapsed).unwrap();
    write_passed(&mut stdout, summary.passed);
    write_test_infos(&mut stdout, &summary.skipped, "Skipped", Color::Yellow);
    write_test_infos(&mut stdout, &summary.pending, "Pending", Color::Yellow);
    write_test_infos(&mut stdout, &summary.undefined, "Undefined", Color::Red);
    write_test_infos(&mut stdout, &summary.ambiguous, "Ambiguous", Color::Red);
    write_test_infos(&mut stdout, &summary.failed, "Failed", Color::Red);

    writeln!(&mut stdout).unwrap();
}


fn write_passed(stdout: &mut StandardStream, passed: usize) {
    let has_passed_tests = passed > 0;

    if has_passed_tests {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
    }
    stdout.write_all("    Passed: ".as_bytes()).unwrap();
    stdout.write_all(passed.to_string().as_bytes()).unwrap();
    stdout.write_all(b"\n").unwrap();
    if has_passed_tests {
        stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
    }
}

fn write_test_infos(stdout: &mut StandardStream, test_infos: &[TestInfo],
    status_name: &str, color: Color)
{
    let has_scenario_designations = !test_infos.is_empty();

    if has_scenario_designations {
        stdout.set_color(ColorSpec::new().set_fg(Some(color))).unwrap();
    }

    stdout.write_all(b"    ").unwrap();
    stdout.write_all(status_name.as_bytes()).unwrap();
    stdout.write_all(b": ").unwrap();
    stdout.write_all(test_infos.len().to_string().as_bytes()).unwrap();

    for test_info in test_infos {
        stdout.write_all(b"\n").unwrap();
        stdout.write_all(b"        ").unwrap();
        stdout.write_all(test_info.keyword.as_bytes()).unwrap();
        stdout.write_all(b": ").unwrap();
        stdout.write_all(test_info.name.as_bytes()).unwrap();
        stdout.write_all(b" \x1B[90m").unwrap();
        stdout.write_all(b"# ").unwrap();
        stdout.write_all(test_info.uri.as_bytes()).unwrap();
        stdout.write_all(b":").unwrap();
        stdout.write_all(test_info.line.to_string().as_bytes()).unwrap();
        stdout.set_color(ColorSpec::new().set_fg(Some(color))).unwrap();
    }

    stdout.write_all(b"\n").unwrap();
    if has_scenario_designations {
        stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
    }
}
