use serde::Serialize;

use cuke_runner::gherkin;
use cuke_runner::api;
use cuke_runner::glue::step::argument::StepArgument;
use std::time::Duration;
use super::serde_nanos;

#[derive(Debug, Serialize)]
pub struct Feature {
    pub id: String,
    pub uri: String,
    pub keyword: String,
    pub name: String,
    pub line: u32,
    pub description: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub elements: Vec<Element>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Serialize)]
pub struct Element {
    pub id: String,
    pub keyword: String,
    pub name: String,
    pub line: u32,
    pub description: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub steps: Vec<Step>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub before: Vec<Hook>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub after: Vec<Hook>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Serialize)]
pub struct Step {
    pub keyword: String,
    pub name: String,
    pub line: u32,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub glue_location: Option<GlueCodeLocation>,
    pub result: Result,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<Row>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_string: Option<DocString>,
}

#[derive(Debug, Serialize)]
pub struct GlueCodeLocation {
    pub location: String,
}

#[derive(Debug, Serialize)]
pub struct Result {
    pub status: String,
    #[serde(with = "serde_nanos", skip_serializing_if = "is_duration_zero")]
    pub duration: Duration,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Row {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cells: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DocString {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub media_type: String,
    pub value: String,
    pub line: u32,
}

#[derive(Debug, Serialize)]
pub struct Hook {
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub glue_code_location: Option<GlueCodeLocation>,
    pub result: Result,
}

#[derive(Debug, Serialize)]
pub struct Tag {
    pub name: String,
    pub location: Location,
}

#[derive(Debug, Serialize)]
pub struct Location {
    pub line: u32,
    pub column: u32,
}

impl From<(&dyn api::CukeStepTestStep<'_>, &api::TestResult)> for Step {
    fn from((cuke_step_test_step, test_result): (&dyn api::CukeStepTestStep<'_>, &api::TestResult)) -> Self {
        Step {
            keyword: cuke_step_test_step.get_step_keyword().to_owned(),
            name: cuke_step_test_step.get_step_text().to_owned(),
            line: cuke_step_test_step.get_step_line(),
            glue_location: cuke_step_test_step.get_glue_code_location().map(|location| GlueCodeLocation::from(location)),
            result: Result::from(test_result),
            rows: rows_from(cuke_step_test_step.get_arguments()),
            doc_string: doc_string_from(cuke_step_test_step.get_arguments()),
        }
    }
}

impl From<(&dyn api::HookTestStep<'_>, &api::TestResult)> for Hook {
    fn from((cuke_step_test_step, test_result): (&dyn api::HookTestStep<'_>, &api::TestResult)) -> Self {
        Hook {
            glue_code_location: cuke_step_test_step.get_glue_code_location().map(|location| GlueCodeLocation::from(location)),
            result: Result::from(test_result),
        }
    }
}

impl From<&api::TestResult> for Result {
    fn from(test_result: &api::TestResult) -> Self {
        Result {
            status: test_result.status.to_string().to_lowercase(),
            duration: test_result.duration.unwrap_or(Duration::new(0, 0)),
            error_message: test_result.get_error_message(),
        }
    }
}

fn rows_from(arguments: &[StepArgument<'_>]) -> Vec<Row> {
    arguments.iter()
        .find_map(|argument| {
            match argument {
                StepArgument::Expression(_expression) => None,
                StepArgument::DocString(_doc_string) => None,
                StepArgument::DataTable(data_table) => {
                    let rows = data_table.iter()
                        .map(|cells_iter| Row {
                            cells: cells_iter
                                .map(|cell| cell.to_owned())
                                .collect::<Vec<String>>(),
                        })
                        .collect();
                    Some(rows)
                },
            }
        })
        .unwrap_or(Vec::new())
}

fn doc_string_from(arguments: &[StepArgument<'_>]) -> Option<DocString> {
    arguments.iter()
        .find_map(|argument| {
            match argument {
                StepArgument::Expression(_expression) => None,
                StepArgument::DataTable(_data_table) => None,
                StepArgument::DocString(doc_string) => {
                    let doc_string = DocString {
                        media_type: doc_string.media_type().to_owned(),
                        value: doc_string.value().to_owned(),
                        line: doc_string.line(),
                    };
                    Some(doc_string)
                },
            }
        })
}

impl From<&api::GlueCodeLocation> for GlueCodeLocation {
    fn from(location: &api::GlueCodeLocation) -> Self {
        GlueCodeLocation {
            location: format!("{}:{}", location.file_path().display(), location.line_number()),
        }
    }
}

impl From<&gherkin::cuke::Tag<'_>> for Tag {
    fn from(tag: &gherkin::cuke::Tag<'_>) -> Self {
        Tag {
            name: tag.name.to_owned(),
            location: Location::from(&tag.location),
        }
    }
}

impl From<&gherkin::ast::Tag> for Tag {
    fn from(tag: &gherkin::ast::Tag) -> Self {
        Tag {
            name: tag.name.clone(),
            location: Location::from(&tag.location.unwrap()),
        }
    }
}

impl From<&gherkin::cuke::Location> for Location {
    fn from(location: &gherkin::cuke::Location) -> Self {
        Location {
            line: location.line,
            column: location.column,
        }
    }
}

impl From<&gherkin::ast::Location> for Location {
    fn from(location: &gherkin::ast::Location) -> Self {
        Location {
            line: location.line,
            column: location.column,
        }
    }
}

pub fn is_duration_zero(duration: &Duration) -> bool {
    duration.as_secs() == 0 && duration.subsec_nanos() == 0
}
