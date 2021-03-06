use std::cell::RefCell;
use std::cmp;
use std::ops::Deref;

use unicode_segmentation::UnicodeSegmentation;

use cuke_runner::api::{CukeStepTestStep, GlueCodeLocation, TestCase, TestResult, TestStep};
use cuke_runner::api::event::{Event, EventListener};
use cuke_runner::gherkin::ast::{Argument, Background, Examples, Feature, Scenario, Tag};
use cuke_runner::gherkin::cuke;
use cuke_runner::glue::step::argument::StepArgument;

const SCENARIO_INDENT: &str = "  ";
const STEP_INDENT: &str = "    ";
const ATTACHED_STEP_ARGUMENT_INDENT: &str = "      ";
const EXAMPLES_INDENT: &str = "    ";
const ERROR_INDENT: &str = "      ";

/// Prints the cuke-runner tests progress in colored textual form to the console (stdout).
///
/// The `pretty_print` feature needs to be enabled to use this event listener.
///
/// To prevent interleaved garbage text this listener does not implement `Sync`
/// and thus can only be used with `ExecutionMode::Sequential`.
///
/// # Examples
///
/// ```rust
/// use cuke_runner_listener::PrettyPrintListener;
///
/// let event_listeners = &[
///     &PrettyPrintListener::new(),
/// ];
/// ```
///
/// <script src="https://asciinema.org/a/MhaLm4GcHKbYtkAYL17UqtvVy.js" id="asciicast-MhaLm4GcHKbYtkAYL17UqtvVy" async data-cols="150"></script>
#[derive(Debug, Default)]
pub struct PrettyPrintListener {
    inner: RefCell<Inner>,
}

#[derive(Debug)]
struct Inner {
    first_feature: bool,
    print_feature_file_text: bool,
    print_scenario_text: bool,
    current_scenario_outline: Option<u32>,
    current_examples: Option<u32>,
    location_indentation: usize,
}

impl Default for Inner {
    fn default() -> Self {
        Inner {
            first_feature: true,
            print_feature_file_text: true,
            print_scenario_text: false,
            current_scenario_outline: None,
            current_examples: None,
            location_indentation: 0,
        }
    }
}

impl PrettyPrintListener {
    pub fn new() -> PrettyPrintListener {
        PrettyPrintListener::default()
    }
}

impl EventListener for PrettyPrintListener {
    fn on_event(&self, event: &Event<'_, '_>) {
        match *event {
            Event::TestCaseStarted {
                uri,
                feature,
                feature_background,
                rule_background,
                scenario,
                test_case,
                ..
            } => self.inner.borrow_mut()
                .handle_test_case_started(
                    uri,
                    feature,
                    feature_background,
                    rule_background,
                    scenario,
                    test_case,
                ),
            Event::TestStepStarted {
                uri,
                scenario,
                test_case,
                test_step,
                ..
            } => self.inner.borrow_mut()
                .handle_test_step_started(uri, scenario, test_case, test_step),
            Event::TestStepFinished {
                test_step,
                result,
                ..
            } => self.inner.borrow_mut()
                .handle_test_step_finished(test_step, result),
            Event::Write {
                text,
                ..
            } => self.inner.borrow()
                .handle_write(text),
            Event::TestRunFinished { .. } => println!(),
            _ => {},
        }
    }
}

impl Inner {
    fn handle_test_case_started(
        &mut self,
        uri: &str,
        feature: &Feature,
        feature_background: Option<&Background>,
        _rule_background: Option<&Background>,
        scenario: &Scenario,
        test_case: &dyn TestCase,
    ) {
        self.handle_start_of_feature(feature);
        self.handle_scenario_outline(uri, scenario, test_case);

        if let Some(background) = feature_background {
            self.print_background(uri, background, test_case);
            self.print_scenario_text = true;
        } else {
            self.print_scenario(uri, scenario, test_case);
        }
    }

    fn print_background(&mut self, uri: &str, background: &Background, test_case: &dyn TestCase) {
        let definition_text = format!("{}: {}", background.keyword, background.name);
        let background_line = background.location.unwrap().line;
        let description = &background.description;
        self.calculate_location_indentation(&definition_text, &test_case.get_test_steps(), true);
        let location_padding = self.create_padding_to_location(SCENARIO_INDENT, &definition_text);
        println!();
        let location_text = self.format_uri_location(uri, background_line);
        println!("{}", SCENARIO_INDENT.to_owned() + &definition_text + &location_padding + &location_text);
        self.print_description(description);
    }

    fn handle_test_step_started(&mut self, uri: &str, scenario: &Scenario, test_case: &dyn TestCase, test_step: &TestStep<'_, '_>) {
        if let TestStep::Cuke(cuke_step_test_step) = test_step {
            if self.print_scenario_text {
                if !cuke_step_test_step.is_background_step() {
                    self.print_scenario(uri, scenario, test_case.deref());
                    self.print_scenario_text = false;
                } else {
                    self.print_scenario_text = true;
                }
            }
        }
    }

    fn handle_test_step_finished(&mut self, test_step: &TestStep<'_, '_>, result: &TestResult) {
        if let TestStep::Cuke(cuke_step_test_step) = test_step {
            self.print_step(*cuke_step_test_step, result);
        }
        self.print_error(result);
    }

    fn handle_write(&self, text: &str) {
        println!("{}", text);
    }

    fn print_step(&self, test_step: &dyn CukeStepTestStep<'_>, result: &TestResult) {
        let keyword = test_step.get_step_keyword();
        let step_text = test_step.get_step_text();
        let definition_text = format!("{}{}", keyword, step_text);
        let location_padding = self.create_padding_to_location(STEP_INDENT, &definition_text);
        let arguments = test_step.get_arguments();
        let formatted_step_text = self.format_step_text(&keyword, step_text, result.status.ansi_color_code(), arguments);
        let location = self.format_glue_code_location(test_step.get_glue_code_location());
        println!("{}", STEP_INDENT.to_owned() + &formatted_step_text + &location_padding + &location);

        if let Some(attached_step_text) = self.format_attached_step_arguments(arguments) {
            println!("\x1B[{}m{}\x1B[0m", result.status.ansi_color_code(), attached_step_text);
        }
    }

    fn format_step_text(&self, keyword: &str, step_text: &str,
        ansi_color_code: u8, arguments: &[StepArgument<'_>]) -> String
    {
        let mut result = format!("\x1B[{}m{}\x1B[0m", ansi_color_code, keyword);
        let mut begin_index = 0usize;

        for argument in arguments {
            let expression = match argument {
                StepArgument::Expression(expression) => expression,
                _ => continue,
            };

            let arg_start = expression.start();
            let arg_end = expression.end();

            // a nested argument starts before the enclosing argument ends; ignore it when formatting
            if arg_start < begin_index {
                continue;
            }

            let text_before_arg = &step_text[begin_index..arg_start];
            result.push_str(&format!("\x1B[{}m{}\x1B[0m", ansi_color_code, text_before_arg));

            let arg_text = &step_text[arg_start..arg_end];
            result.push_str(&format!("\x1B[{};1m{}\x1B[0m", ansi_color_code, arg_text));
            begin_index = arg_end;
        }

        if begin_index != step_text.len() {
            let text_after_args = &step_text[begin_index..step_text.len()];
            result.push_str(&format!("\x1B[{}m{}\x1B[0m", ansi_color_code, text_after_args));
        }

        result
    }

    fn format_attached_step_arguments(&self, arguments: &[StepArgument<'_>]) -> Option<String> {
        for argument in arguments {
            match argument {
                StepArgument::Expression(_expression) => {},
                StepArgument::DocString(doc_string) => {
                    return Some(self.format_doc_string(doc_string.value()));
                },
                StepArgument::DataTable(data_table) => {
                    return Some(self.format_data_table(data_table.iter()));
                },
            }
        }

        None
    }

    fn format_doc_string(&self, doc_string: &str) -> String {
        let mut result = String::new();
        result.push_str(ATTACHED_STEP_ARGUMENT_INDENT);
        result.push_str("\"\"\"\n");
        for line in doc_string.lines() {
            result.push_str(ATTACHED_STEP_ARGUMENT_INDENT);
            result.push_str(line);
            result.push('\n');
        }
        result.push_str(ATTACHED_STEP_ARGUMENT_INDENT);
        result.push_str("\"\"\"");
        result
    }

    fn format_data_table<'a>(&self, data_table_iter: impl Iterator<Item=impl Iterator<Item=&'a str>>)
        -> String
    {
        let mut result = String::new();

        let data_table = data_table_iter
            .map(|cells_iter| cells_iter.collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        let mut max_column_graphemes_counts = vec![0; data_table[0].len()];
        for row in &data_table {
            for (index, cell_value) in row.iter().enumerate() {
                let graphemes_count = UnicodeSegmentation::graphemes(*cell_value, true)
                    .count();
                let prev_max = max_column_graphemes_counts[index];
                max_column_graphemes_counts[index] = std::cmp::max(prev_max, graphemes_count);
            }
        }

        let mut first_row = true;
        for row in &data_table {
            if first_row {
                first_row = false;
            } else {
                result.push('\n');
            }

            result.push_str(ATTACHED_STEP_ARGUMENT_INDENT);

            let mut first_column = true;
            for (index, cell_value) in row.iter().enumerate() {
                if first_column {
                    result.push('|');
                    first_column = false;
                }

                let graphemes_count = UnicodeSegmentation::graphemes(*cell_value, true)
                    .count();
                let indent = " ".repeat(max_column_graphemes_counts[index] - graphemes_count);

                result.push(' ');
                result.push_str(cell_value);
                result.push_str(&indent);
                result.push_str(" |");
            }
        }

        result
    }

    fn print_error(&self, result: &TestResult) {
        if let Some(error_message) = &result.get_error_message() {
            let error_line_indention = "\n".to_owned() + ERROR_INDENT;
            let mut message = error_message.replace('\n', &error_line_indention);
            message.insert_str(0, &ERROR_INDENT);
            println!("\x1B[{}m{}\x1B[0m\n", result.status.ansi_color_code(), message)
        }
    }

    fn handle_start_of_feature(&mut self, feature: &Feature) {
        if self.print_feature_file_text {
            if !self.first_feature {
                println!();
            }

            self.print_feature(feature);
            self.print_feature_file_text = false;
            self.first_feature = false;
        }
    }

    fn print_feature(&self, feature: &Feature) {
        self.print_tags(&feature.tags);
        println!("{}: {}", feature.keyword, feature.name);
        self.print_description(&feature.description);
    }

    fn handle_scenario_outline(&mut self, uri: &str, scenario: &Scenario, test_case: &dyn TestCase) {
        if !scenario.examples.is_empty() {
            let scenario_outline_line = scenario.location.unwrap().line;
            let mut reset_scenario_outline = false;

            {
                if self.current_scenario_outline.is_none()
                    || self.current_scenario_outline.unwrap() != scenario_outline_line {
                    self.print_scenario_outline(uri, scenario);
                    reset_scenario_outline = true;
                }
            }

            let test_case_line = test_case.get_line();

            let mut current_examples = None;
            for examples in &scenario.examples {
                if examples.location.unwrap().line == test_case_line {
                    current_examples = Some(examples);
                    break;
                }

                if let Some(table_header) = &examples.table_header {
                    if table_header.location.unwrap().line == test_case_line {
                        current_examples = Some(examples);
                        break;
                    }
                }

                for table_row in &examples.table_body {
                    if table_row.location.unwrap().line == test_case_line {
                        current_examples = Some(examples);
                        break;
                    }
                }
            }

            let current_examples = current_examples.expect("current examples");

            if self.current_examples.is_none()
                || self.current_examples.unwrap() != current_examples.location.unwrap().line {
                self.print_examples(current_examples);
                self.current_examples = Some(current_examples.location.unwrap().line);
            }

            if reset_scenario_outline {
                self.current_scenario_outline = Some(scenario_outline_line);
            }
        } else {
            self.current_scenario_outline = None;
            self.current_examples = None;
        }
    }

    fn print_examples(&self, examples: &Examples) {
        println!();
        self.print_tags_with_ident(&examples.tags, EXAMPLES_INDENT);
        println!("{}", EXAMPLES_INDENT.to_owned() + &examples.keyword + ": " + &examples.name);
        self.print_description(&examples.description);

        let examples_table_iter = examples.table_header
            .iter()
            .chain(examples.table_body.iter())
            .map(|table_row| {
                table_row.cells
                    .iter()
                    .map(|table_cell| table_cell.value.as_str())
            });
        println!("\x1B[36m{}\x1B[0m", self.format_data_table(examples_table_iter));
    }

    fn print_scenario_outline(&self, uri: &str, scenario: &Scenario) {
        println!();
        self.print_tags_with_ident(&scenario.tags, SCENARIO_INDENT);
        let definition_text = format!("{}: {}",
            scenario.keyword, scenario.name);
        let location_text = self.format_uri_location(uri, scenario.location.unwrap().line);
        println!("{}", SCENARIO_INDENT.to_owned() + &definition_text + " " + &location_text);
        self.print_description(&scenario.description);
        for step in &scenario.steps {
            let step_text = format!("\x1B[36m{}{}\x1B[0m", step.keyword, step.text);
            println!("{}", STEP_INDENT.to_owned() + &step_text);

            if let Some(attached_argument) = self.format_attached_argument(&step.argument) {
                println!("\x1B[36m{}\x1B[0m", attached_argument);
            }
        }
    }

    fn format_attached_argument(&self, argument: &Option<Argument>) -> Option<String> {
        argument.as_ref().map(|argument| {
            match argument {
                Argument::DocString(doc_string) => {
                    self.format_doc_string(&doc_string.content)
                },
                Argument::DataTable(data_table) => {
                    let data_table_iter = data_table.rows
                        .iter()
                        .map(|row| {
                            row.cells
                                .iter()
                                .map(|cell| cell.value.as_str())
                        });
                    self.format_data_table(data_table_iter)
                },
            }
        })
    }

    fn print_tags(&self, tags: &[Tag]) {
        self.print_tags_with_ident(tags, "");
    }

    fn print_tags_with_ident(&self, tags: &[Tag], indent: &str) {
        if !tags.is_empty() {
            let tag_names: Vec<&str> = tags.iter()
                .map(|tag| tag.name.as_ref())
                .collect();
            println!("{}{}", indent, tag_names.join(" "));
        }
    }

    fn print_cuke_tags(&self, tags: &[cuke::Tag<'_>], indent: &str) {
        if !tags.is_empty() {
            let tag_names: Vec<&str> = tags.iter()
                .map(cuke::Tag::as_ref)
                .collect();
            println!("{}{}", indent, tag_names.join(" "));
        }
    }

    fn print_description<S: AsRef<str>>(&self, description: S) {
        let description = description.as_ref();
        if !description.is_empty() {
            println!("{}", description);
        }
    }

    fn print_scenario(&mut self, uri: &str, scenario: &Scenario,
        test_case: &dyn TestCase)
    {
        let definition_text = format!("{}: {}",
            scenario.keyword, test_case.get_name());
        let test_steps = &test_case.get_test_steps();
        let description = &scenario.description;
        self.calculate_location_indentation(&definition_text, test_steps, false);
        let location_padding = self.create_padding_to_location(SCENARIO_INDENT, &definition_text);
        println!();
        self.print_cuke_tags(test_case.get_tags(), SCENARIO_INDENT);
        let location_text = self.format_uri_location(uri, test_case.get_line());
        println!("{}", SCENARIO_INDENT.to_owned() + &definition_text + &location_padding + &location_text);
        self.print_description(description);
    }

    fn calculate_location_indentation(&mut self, definition_text: &str,
        test_steps: &[TestStep<'_, '_>], use_background_steps: bool) {

        let mut max_text_length = SCENARIO_INDENT.chars().count() + definition_text.chars().count();
        for step in test_steps {
            if let TestStep::Cuke(test_step) = step.deref() {
                if test_step.is_background_step() == use_background_steps {
                    let step_text = self.step_text(*test_step);
                    max_text_length = cmp::max(max_text_length, STEP_INDENT.chars().count() + step_text.chars().count());
                }
            }
        }

        max_text_length += 1;
        self.location_indentation = max_text_length;
    }

    fn step_text(&self, test_step: &dyn CukeStepTestStep<'_>) -> String {
        let keyword = test_step.get_step_keyword();
        let text = test_step.get_step_text();
        format!("{}{}", keyword, text)
    }

    fn create_padding_to_location(&self, indent: &str, text: &str) -> String {
        let mut padding = String::new();

        for _ in indent.chars().count() + text.chars().count()..self.location_indentation as usize {
            padding.push(' ');
        }

        padding
    }

    fn format_uri_location(&self, uri: &str, line: u32) -> String {
        let location = format!("{}:{}", uri, line);
        format!("\x1B[90m# {}\x1B[0m", location)
    }

    fn format_glue_code_location(&self, location: Option<&GlueCodeLocation>) -> String {
        if let Some(location) = location {
            format!("\x1B[90m# {}\x1B[0m", location)
        } else {
            String::new()
        }
    }

}

#[cfg(test)]
mod tests {
    use super::PrettyPrintListener;

    fn assert_send<T: Send>() {}

    #[test]
    fn test_send() {
        assert_send::<PrettyPrintListener>();
    }
}
