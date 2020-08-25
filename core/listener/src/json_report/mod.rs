use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Write;
use std::sync::Mutex;

use gherkin::ast::{Background, Scenario};

use cuke_runner::api::{HookType, TestCase, TestResult, TestStep};
use cuke_runner::api::event::{Event, EventListener};
use cuke_runner::gherkin;
use data::*;

mod serde_nanos;
mod data;

/// Creates a json report.
///
/// The format of the report is not specified,
/// but it aims to follow the format of the [`cucumber-jvm implementation`].
/// The generated json report can be used as source for html report generators
/// like the [`cluecumber-report-plugin`].
///
/// The `json_report` feature needs to be enabled to use this event listener.
///
/// This listener implements `Sync` and thus can be used in parallel execution modes.
///
/// # Examples
///
/// ```rust,no_run
/// use std::path::PathBuf;
/// use cuke_runner_listener::JsonReportListener;
///
/// let output_dir = &[
///     env!("CARGO_MANIFEST_DIR"),
///     "target",
///     "cucumber",
/// ].iter().collect::<PathBuf>();
///
/// std::fs::create_dir_all(&output_dir).unwrap();
/// let json_report_path = output_dir.join("report.json");
/// let mut json_report_file = std::fs::File::create(json_report_path).unwrap();
///
/// let event_listeners = &[
///     &JsonReportListener::with_writer(&mut json_report_file),
/// ];
/// ```
///
/// [`cucumber-jvm implementation`]: https://github.com/cucumber/cucumber-jvm/blob/master/core/src/main/java/io/cucumber/core/plugin/JSONFormatter.java
/// [`cluecumber-report-plugin`]: https://github.com/trivago/cluecumber-report-plugin
#[derive(Debug)]
pub struct JsonReportListener<W: Write + Send + Debug> {
    report: Mutex<RefCell<Report<W>>>,
}

#[derive(Debug, Default)]
struct Report<W: Write + Send + Debug> {
    features: HashMap<String, Feature>,
    id_count: usize,
    writer: W,
}

impl<W: Write + Send + Debug> JsonReportListener<W> {
    pub fn with_writer(writer: W) -> JsonReportListener<W> {
        JsonReportListener {
            report: Mutex::new(RefCell::new(Report {
                features: HashMap::new(),
                id_count: 0,
                writer,
            })),
        }
    }
}

impl<W: Write + Send + Debug> EventListener for JsonReportListener<W> {
    fn on_event(&self, event: &Event<'_, '_>) {
        match *event {
            Event::TestSourceRead { uri, feature, .. } => {
                let report_lock = self.report.lock().unwrap();
                let mut report = report_lock.borrow_mut();
                report.add_feature(uri, feature);
            }
            Event::TestStepFinished {
                uri,
                feature_background,
                rule_background,
                scenario,
                test_case,
                test_step,
                result,
                ..
            } => {
                let report_lock = self.report.lock().unwrap();
                let mut report = report_lock.borrow_mut();
                report.add_test_step_result(uri, feature_background, rule_background, scenario,
                    test_case, test_step, result);
            }
            Event::TestRunFinished { .. } => {
                let report_lock = self.report.lock().unwrap();
                let mut report = report_lock.borrow_mut();
                report.write();
            }
            _ => {}
        }
    }
}

impl<W: Write + Send + Debug> Report<W> {
    fn add_feature(&mut self, uri: &str, feature: &gherkin::ast::Feature) {
        self.id_count += 1;
        let id = format!("feature-{}", self.id_count);

        self.features.insert(uri.to_string(), Feature {
            id,
            uri: uri.to_string(),
            keyword: feature.keyword.to_owned(),
            name: feature.name.to_owned(),
            line: feature.location.unwrap().line,
            description: feature.description.to_owned(),
            elements: Vec::new(),
            tags: feature.tags.iter().map(Tag::from).collect(),
        });
    }

    fn add_test_step_result(&mut self, uri: &str,
        feature_background: Option<&Background>, _rule_background: Option<&Background>,
        scenario: &Scenario, test_case: &dyn TestCase, test_step: &TestStep<'_>,
        result: &TestResult)
    {
        let mut new_id_count = self.id_count + 1;
        let feature = self.features.get_mut(uri).unwrap();

        match test_step {
            TestStep::Hook(hook_test_step) => {
                let ty = match hook_test_step.get_hook_type() {
                    HookType::BeforeScenario => "before",
                    HookType::AfterScenario => "after",
                    HookType::BeforeStep => unimplemented!(),
                    HookType::AfterStep => unimplemented!(),
                };

                let element = if let Some(last_element) = feature.elements.last_mut() {
                    if &last_element.ty == ty {
                        new_id_count -= 1;
                        last_element
                    } else {
                        let id = format!("element-{}", new_id_count);
                        feature.elements.push(create_element_from_scenario(id, scenario, test_case, ty));
                        feature.elements.last_mut().unwrap()
                    }
                } else {
                    let id = format!("element-{}", new_id_count);
                    feature.elements.push(create_element_from_scenario(id, scenario, test_case, ty));
                    feature.elements.last_mut().unwrap()
                };

                let hook = Hook::from((*hook_test_step, result));
                match hook_test_step.get_hook_type() {
                    HookType::BeforeScenario => {
                        element.before.push(hook)
                    },
                    HookType::AfterScenario => {
                        element.after.push(hook)
                    },
                    HookType::BeforeStep => unimplemented!(),
                    HookType::AfterStep => unimplemented!(),
                };
            }
            TestStep::Cuke(cuke_step_test_step) => {
                let element = if cuke_step_test_step.is_background_step() {
                    let ty = "background";

                    if let Some(last_element) = feature.elements.last_mut() {
                        if &last_element.ty == ty {
                            new_id_count -= 1;
                            last_element
                        } else {
                            let background = feature_background.unwrap();
                            let id = format!("element-{}", new_id_count);
                            feature.elements.push(create_element_from_background(id, background, ty));
                            feature.elements.last_mut().unwrap()
                        }
                    } else {
                        let background = feature_background.unwrap();
                        let id = format!("element-{}", new_id_count);
                        feature.elements.push(create_element_from_background(id, background, ty));
                        feature.elements.last_mut().unwrap()
                    }
                } else {
                    let ty = "scenario";

                    if let Some(last_element) = feature.elements.last_mut() {
                        if &last_element.ty == ty {
                            new_id_count -= 1;
                            last_element
                        } else {
                            let id = format!("element-{}", new_id_count);
                            feature.elements.push(create_element_from_scenario(id, scenario, test_case, ty));
                            feature.elements.last_mut().unwrap()
                        }
                    } else {
                        let id = format!("element-{}", new_id_count);
                        feature.elements.push(create_element_from_scenario(id, scenario, test_case, ty));
                        feature.elements.last_mut().unwrap()
                    }
                };

                let step = Step::from((*cuke_step_test_step, result));
                element.steps.push(step);
            },
        }

        self.id_count = new_id_count;
    }

    fn write(&mut self) {
        let features = self.features
            .drain()
            .map(|(_uri, feature)| feature)
            .collect::<Vec<Feature>>();
        serde_json::to_writer(&mut self.writer, &features).unwrap();
    }
}



fn create_element_from_background(id: String, background: &Background, ty: &str)
    -> Element
{
    Element {
        id,
        keyword: background.keyword.to_owned(),
        name: background.name.to_owned(),
        line: background.location.unwrap().line,
        description: background.description.to_owned(),
        ty: ty.to_owned(),
        steps: Vec::new(),
        before: Vec::new(),
        after: Vec::new(),
        tags: Vec::new(),
    }
}

fn create_element_from_scenario(id: String, scenario: &Scenario,
    test_case: &dyn TestCase, ty: &str) -> Element
{
    Element {
        id,
        keyword: scenario.keyword.to_owned(),
        name: test_case.get_name().to_owned(),
        line: test_case.get_line(),
        description: scenario.description.to_owned(),
        ty: ty.to_owned(),
        steps: Vec::new(),
        before: Vec::new(),
        after: Vec::new(),
        tags: test_case.get_tags().iter().map(Tag::from).collect(),
    }
}



#[cfg(test)]
mod tests {
    use super::JsonReportListener;

    fn assert_sync<T: Sync>() {}

    fn assert_send<T: Send>() {}

    #[test]
    fn test_send_sync() {
        assert_send::<JsonReportListener<std::fs::File>>();
        assert_sync::<JsonReportListener<std::fs::File>>();
    }
}
