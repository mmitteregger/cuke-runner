//use std::time::Duration;
use std::any::{TypeId, Any};
use std::fmt;

use gherkin::pickle::{PickleStep, PickleString, PickleTable, PickleRow, PickleCell, PickleArgument};

use api::SourceCodeLocation;
use glue::StepFn;
use runtime::Scenario;
use super::step_expression::{StepExpression, Argument};

#[derive(Clone)]
pub struct StepDefinition {
    pub expression: StepExpression,
    pub parameter_infos: Vec<TypeId>,
//    pub timeout: Duration,
    pub step_fn: StepFn,
    pub location: SourceCodeLocation,
}

impl fmt::Debug for StepDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        f.debug_struct("StepDefinition")
            .field("expression", &self.expression)
            .field("parameter_infos", &self.parameter_infos)
//            .field("timeout", &self.timeout)
            .field("step_fn", &"<step_fn>")
            .field("location", &self.location)
            .finish()
    }
}

impl StepDefinition {
    /// Returns the list of arguments for this step definition.
    ///
    /// Returns `None` if the step definition doesn't match at all.
    /// Returns an empty `Vec` if it matches with 0 arguments
    /// and bigger sizes if it matches several.
    pub fn matched_arguments(&self, step: &PickleStep) -> Option<Vec<Argument>> {
        let mut matched_arguments = match self.expression.matched_arguments(&step.text) {
            Some(arguments) => arguments,
            None => return None,
        };

        if step.arguments.is_empty() {
            Some(matched_arguments)
        } else {
            debug_assert!(step.arguments.len() == 1);
            let argument = step.arguments.first().unwrap();

            matched_arguments.reserve_exact(1);

            match argument {
                PickleArgument::String(pickle_string) =>
                    matched_arguments.push(Argument::DocString(pickle_string.content.clone())),
                PickleArgument::Table(pickle_table) =>
                    matched_arguments.push(Argument::DataTable),
                _ => eprintln!("Ignoring unknown step argument: {:?}", argument),
            }

            Some(matched_arguments)
        }
    }

    /// The source line where the step definition is defined.
    ///
    /// Example: foo/bar/Zap.brainfuck:42
    pub fn get_location(&self) -> &SourceCodeLocation {
        &self.location
    }

    /// The number of declared parameters of this step definition.
    pub fn get_parameter_count(&self) -> u8 {
        self.parameter_infos.len() as u8
    }

    /// Invokes the step definition.
    pub fn execute(&self, scenario: &mut Scenario, args: Vec<Box<Any>>) -> ::std::result::Result<(), ::glue::ExecutionError> {
        let result = (self.step_fn)(&mut scenario.glue_scenario);
        result

    }

    /// The step definition pattern for error reporting only.
    pub fn get_pattern(&self) -> &String {
        unimplemented!();
    }
}
