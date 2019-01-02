// Perf notes: The code in this module is *not* yett written with performance in mind
// It is hacked together to print something useful,
// but there are lots of unnecessary clones and loops that should be avoided!

use std::collections::HashMap;
use std::rc::Rc;
use std::cmp;
use std::ops::Deref;

use gherkin::ast::*;
use gherkin::pickle::*;

use api::event::{Event, EventListener};
use api::{TestCase, TestStep, TestResult, Argument, CodeLocation};
use runner::PickleStepTestStep;

const SCENARIO_INDENT: &str = "  ";
const STEP_INDENT: &str = "    ";
const EXAMPLES_INDENT: &str = "    ";
const ERROR_INDENT: &str = "      ";

#[derive(Debug, Default)]
pub struct PrettyFormatter {
    features: HashMap<String, Rc<Feature>>,
    current_feature_file: Option<String>,
    current_test_case: Option<Rc<TestCase>>,
    current_scenario_outline: Option<u32>,
    current_examples: Option<u32>,
    location_indentation: usize,
}

impl PrettyFormatter {
    pub fn new() -> PrettyFormatter {
        PrettyFormatter::default()
    }
}

impl EventListener for PrettyFormatter {
    fn on_event(&mut self, event: &Event) {
        match *event {
            Event::TestSourceRead {
                uri,
                feature,
                ..
            } => self.handle_test_source_read(uri, feature),
            Event::TestCaseStarted {
                test_case,
                ..
            } => self.handle_test_case_started(test_case),
            Event::TestStepStarted {
                test_step,
                ..
            } => self.handle_test_step_started(test_step),
            Event::TestStepFinished {
                test_step,
                result,
                ..
            } => self.handle_test_step_finished(test_step, result),
            Event::Write {
                text,
                ..
            } => self.handle_write(text),
            _ => {},
        }
    }
}

impl PrettyFormatter {
    fn handle_test_source_read(&mut self, uri: &str, feature: &Rc<Feature>) {
        self.features.insert(uri.to_owned(), feature.clone());
    }

    fn handle_test_case_started(&mut self, test_case: &Rc<TestCase>) {
        self.handle_start_of_feature(test_case.deref());
        self.handle_scenario_outline(test_case.deref());

        if self.get_background().is_some() {
            self.print_background(test_case.deref());
            self.current_test_case = Some(test_case.clone());
        } else {
            self.print_scenario_definition(test_case.deref());
        }
    }

    fn print_background(&mut self, test_case: &TestCase) {
        if let Some(background) = self.get_background() {
            let definition_text = format!("{}: {}", background.keyword, background.name);
            let background_line = background.location.line;
            let description = background.description.clone();
            self.calculate_location_indentation(&definition_text, &test_case.get_test_steps(), true);
            let location_padding = self.create_padding_to_location(SCENARIO_INDENT, &definition_text);
            println!();
            let location_text = self.format_feature_file_location(background_line);
            println!("{}", SCENARIO_INDENT.to_owned() + &definition_text + &location_padding + &location_text);
            self.print_description(description.as_ref());
        }
    }

    fn handle_test_step_started(&mut self, test_step: &TestStep) {
        if let Some(pickle_step_test_step) = test_step.downcast_ref::<PickleStepTestStep>() {
            if let Some(test_case) = self.current_test_case.take() {
                if !self.is_background_step(pickle_step_test_step) {
                    self.print_scenario_definition(test_case.deref());
                    self.current_test_case = None;
                } else {
                    self.current_test_case = Some(test_case);
                }
            }
        }
    }

    fn handle_test_step_finished(&mut self, test_step: &TestStep, result: &TestResult) {
        if let Some(pickle_step_test_step) = test_step.downcast_ref::<PickleStepTestStep>() {
            self.print_step(pickle_step_test_step, result);
        }
        self.print_error(result);
    }

    fn handle_write(&self, text: &str) {
        println!("{}", text);
    }

    fn print_step(&self, test_step: &PickleStepTestStep, result: &TestResult) {
        use api::PickleStepTestStep;

        let keyword = self.get_step_keyword(test_step);
        let step_text = test_step.get_step_text();
        let definition_text = format!("{}{}", keyword, step_text);
        let location_padding = self.create_padding_to_location(STEP_INDENT, &definition_text);
        let formatted_step_text = self.format_step_text(&keyword, step_text, result.status.ansi_color_code(), test_step.get_definition_argument());
        let location = self.format_code_location(test_step.get_code_location());
        println!("{}", STEP_INDENT.to_owned() + &formatted_step_text + &location_padding + &location);
    }

    fn format_step_text(&self, keyword: &str, step_text: &str,
        ansi_color_code: u8, arguments: Vec<Box<Argument>>) -> String
    {
        let mut result = format!("\x1B[{}m{}\x1B[0m", ansi_color_code, keyword);
        let mut begin_index = 0usize;

        for argument in arguments {
            // the value can be missing if the argument isn't there, for example #[step("(it )?has something")]
            if let Some(arg_value) = argument.value() {
                let arg_start = argument.start();
                let arg_end = argument.end();

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
        }

        if begin_index != step_text.len() {
            let text_after_args = &step_text[begin_index..step_text.len()];
            result.push_str(&format!("\x1B[{}m{}\x1B[0m", ansi_color_code, text_after_args));
        }

        result

    }

    fn print_error(&self, result: &TestResult) {
        if let Some(error_message) = &result.get_error_message() {
            let error_line_indention = "\n".to_owned() + &ERROR_INDENT;
            let mut message = error_message.replace('\n', &error_line_indention);
            message.insert_str(0, &ERROR_INDENT);
            println!("\x1B[{}m{}\x1B[0m\n", result.status.ansi_color_code(), message)
        }
    }

    fn handle_start_of_feature(&mut self, test_case: &TestCase) {
        let uri = test_case.get_uri();

        if self.current_feature_file.is_none() || self.current_feature_file.as_ref().unwrap() != uri {
            if self.current_feature_file.is_some() {
                println!();
            }

            self.current_feature_file = Some(uri.to_owned());
            self.print_feature(uri);
        }
    }

    fn print_feature(&self, path: &str) {
        let feature = self.features.get(path).expect("feature");
        self.print_tags(&feature.tags);
        println!("{}: {}", feature.keyword, feature.name);
        self.print_description(feature.description.as_ref());
    }

    fn handle_scenario_outline(&mut self, test_case: &TestCase) {
        let scenario_definition = self.get_scenario_definition(test_case.get_line());

        if let ScenarioDefinition::ScenarioOutline(scenario_outline) = scenario_definition {
            let scenario_outline_line = scenario_outline.location.line;
            let mut reset_scenario_outline = false;

            {
                if self.current_scenario_outline.is_none()
                    || self.current_scenario_outline.unwrap() != scenario_outline_line {
                    self.print_scenario_outline(scenario_outline);
                    reset_scenario_outline = true;
                }
            }

            let test_case_line = test_case.get_line();

            let mut current_examples = None;
            for examples in &scenario_outline.examples {
                if examples.location.line == test_case_line {
                    current_examples = Some(examples);
                    break;
                }

                if let Some(table_header) = &examples.table_header {
                    if table_header.location.line == test_case_line {
                        current_examples = Some(examples);
                        break;
                    }
                }

                if let Some(table_body) = &examples.table_body {
                    for table_row in table_body {
                        if table_row.location.line == test_case_line {
                            current_examples = Some(examples);
                            break;
                        }
                    }
                }
            }

            let current_examples = current_examples.expect("current examples");

            if self.current_examples.is_none()
                || self.current_examples.unwrap() != current_examples.location.line {
                self.print_examples(current_examples);
                self.current_examples = Some(current_examples.location.line);
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
        self.print_description(examples.description.as_ref());
    }

    fn print_scenario_outline(&self, scenario_outline: &ScenarioOutline) {
        println!();
        self.print_tags_with_ident(&scenario_outline.tags, SCENARIO_INDENT);
        let definition_text = format!("{}: {}",
            scenario_outline.keyword, scenario_outline.name);
        let location_text = self.format_feature_file_location(scenario_outline.location.line);
        println!("{}", SCENARIO_INDENT.to_owned() + &definition_text + " " + &location_text);
        self.print_description(scenario_outline.description.as_ref());
        for step in &scenario_outline.steps {
            let step_text = format!("\x1B[36m{}{}\x1B[0m", step.keyword, step.text);
            println!("{}", STEP_INDENT.to_owned() + &step_text);
        }
    }

    fn print_tags(&self, tags: &[Tag]) {
        self.print_tags_with_ident(tags, "");
    }

    fn print_tags_with_ident(&self, tags: &[Tag], indent: &str) {
        if !tags.is_empty() {
            let tag_names: Vec<&str> = tags.iter()
                .map(|tag| tag.name.as_str())
                .collect();
            println!("{}{}", indent, tag_names.join(" "));
        }
    }

    fn print_pickle_tags(&self, tags: &[PickleTag], indent: &str) {
        if !tags.is_empty() {
            let tag_names: Vec<&str> = tags.iter()
                .map(|tag| tag.name.as_str())
                .collect();
            println!("{}{}", indent, tag_names.join(" "));
        }
    }

    fn print_description(&self, description: Option<&String>) {
        if let Some(description) = description {
            println!("{}", description);
        }
    }

    fn print_scenario_definition(&mut self, test_case: &TestCase) {
        let scenario_definition = self.get_scenario_definition(test_case.get_line());
        let definition_text = format!("{}: {}",
            scenario_definition.get_keyword(), test_case.get_name());
        let test_steps = &test_case.get_test_steps();
        let description = scenario_definition.get_description().cloned();
        self.calculate_location_indentation(&definition_text, test_steps, false);
        let location_padding = self.create_padding_to_location(SCENARIO_INDENT, &definition_text);
        println!();
        self.print_pickle_tags(test_case.get_tags(), SCENARIO_INDENT);
        let location_text = self.format_feature_file_location(test_case.get_line());
        println!("{}", SCENARIO_INDENT.to_owned() + &definition_text + &location_padding + &location_text);
        self.print_description(description.as_ref());
    }

    fn get_scenario_definition(&self, line: u32) -> &ScenarioDefinition {
        let current_feature_file = self.current_feature_file.as_ref().unwrap();
        let feature = self.features.get(current_feature_file).expect("feature");

        for scenario_definition in &feature.scenario_definitions {
            if let ScenarioDefinition::ScenarioOutline(outline) = scenario_definition {
                for examples in &outline.examples {
                    if examples.location.line == line {
                        return scenario_definition;
                    }

                    if let Some(table_header) = &examples.table_header {
                        if table_header.location.line == line {
                            return scenario_definition;
                        }
                    }

                    if let Some(table_body) = &examples.table_body {
                        for table_row in table_body {
                            if table_row.location.line == line {
                                return scenario_definition;
                            }
                        }
                    }
                }
            } else {
                if scenario_definition.get_location().line == line {
                    return scenario_definition;
                }
            }
        }

        panic!("Could not find scenario definition from feature {} at line {}",
            current_feature_file, line);
    }

    fn get_background(&self) -> Option<&Background> {
        let current_feature_file = self.current_feature_file.as_ref().unwrap();
        let feature = self.features.get(current_feature_file).expect("feature");

        if let Some(scenario_definition) = feature.scenario_definitions.first() {
            if let ScenarioDefinition::Background(background) = scenario_definition {
                return Some(background);
            }
        }

        None
    }

    fn calculate_location_indentation(&mut self, definition_text: &str,
        test_steps: &Vec<Box<&TestStep>>, use_background_steps: bool) {

        let mut max_text_length = SCENARIO_INDENT.chars().count() + definition_text.chars().count();
        for step in test_steps {
            if let Some(test_step) = step.downcast_ref::<PickleStepTestStep>() {
                if self.is_background_step(test_step) == use_background_steps {
                    let step_text = self.step_text(test_step);
                    max_text_length = cmp::max(max_text_length, STEP_INDENT.chars().count() + step_text.chars().count());
                }
            }
        }

        max_text_length += 1;
        self.location_indentation = max_text_length;
    }

    fn is_background_step(&self, test_step: &PickleStepTestStep) -> bool {
        use api::PickleStepTestStep;

        let line = test_step.get_step_line();

        let current_feature_file = self.current_feature_file.as_ref().unwrap();
        let feature = self.features.get(current_feature_file).expect("feature");

        for scenario_definition in &feature.scenario_definitions {
            if let ScenarioDefinition::Background(background) = scenario_definition {
                if background.location.line == line {
                    return true;
                }

                for step in &background.steps {
                    if step.location.line == line {
                        return true;
                    }
                }

                return false;
            }
        }

        false
    }

    fn step_text(&self, test_step: &PickleStepTestStep) -> String {
        use api::PickleStepTestStep;

        let keyword = self.get_step_keyword(test_step);
        let mut step_text = keyword;
        step_text.push_str(test_step.get_step_text());
        step_text
    }

    fn get_step_keyword(&self, test_step: &PickleStepTestStep) -> String {
        use api::PickleStepTestStep;

        match self.get_step(test_step.get_step_line()) {
            Some(step) => step.keyword.to_owned(),
            None => String::new(),
        }
    }

    fn get_step(&self, line: u32) -> Option<&Step> {
        let current_feature_file = self.current_feature_file.as_ref().unwrap();
        let feature = self.features.get(current_feature_file).expect("feature");

        for scenario_definition in &feature.scenario_definitions {
            for step in scenario_definition.get_steps() {
                if step.location.line == line {
                    return Some(step);
                }
            }
        }

        None
    }

    fn create_padding_to_location(&self, indent: &str, text: &str) -> String {
        let mut padding = String::new();

        for _ in indent.chars().count() + text.chars().count()..self.location_indentation as usize {
            padding.push(' ');
        }

        padding
    }

    fn format_feature_file_location(&self, line: u32) -> String {
        let location = format!("{}:{}", self.current_feature_file.as_ref().unwrap(), line);
        format!("\x1B[90m# {}\x1B[0m", location)
    }

    fn format_code_location(&self, location: Option<&CodeLocation>) -> String {
        if let Some(location) = location {
            format!("\x1B[90m# {}\x1B[0m", location)
        } else {
            String::new()
        }
    }

}
