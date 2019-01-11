//use std::time::Duration;
use std::any::TypeId;
use std::fmt;

use gherkin::pickle::{PickleArgument, PickleStep};

use api::CodeLocation;
use glue::{StaticStepDefinition, StepFn};
use glue::{StepArgument, DocStringArgument, DataTableArgument};
use runtime::Scenario;

use super::step_expression::StepExpression;

#[derive(Clone)]
pub struct StepDefinition {
    pub expression: StepExpression,
    pub parameter_infos: Vec<TypeId>,
//    pub timeout: Duration,
    pub step_fn: StepFn,
    pub location: CodeLocation,
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

impl From<&&StaticStepDefinition> for StepDefinition {
    fn from(static_step_definition: &&StaticStepDefinition) -> Self {
        StepDefinition {
            expression: StepExpression::from_regex(static_step_definition.expression),
            parameter_infos: Vec::new(),
            step_fn: static_step_definition.step_fn,
            location: static_step_definition.location,
        }
    }
}

impl StepDefinition {
    /// Returns the list of arguments for this step definition.
    ///
    /// Returns `None` if the step definition doesn't match at all.
    /// Returns an empty `Vec` if it matches with 0 arguments
    /// and bigger sizes if it matches several.
    pub fn matched_arguments(&self, step: &PickleStep) -> Option<Vec<StepArgument>> {
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
                    matched_arguments.push(StepArgument::DocString(DocStringArgument::from(pickle_string))),
                PickleArgument::Table(pickle_table) =>
                    matched_arguments.push(StepArgument::DataTable(DataTableArgument::from(pickle_table))),
                _ => eprintln!("Ignoring unknown step argument: {:?}", argument),
            }

            Some(matched_arguments)
        }
    }

    /// The source line where the step definition is defined.
    ///
    /// Example: foo/bar/Zap.brainfuck:42
    pub fn get_location(&self) -> &CodeLocation {
        &self.location
    }

    /// The number of declared parameters of this step definition.
    pub fn get_parameter_count(&self) -> u8 {
        self.parameter_infos.len() as u8
    }

    /// Invokes the step definition.
    pub fn execute(&self, scenario: &mut Scenario, args: &[StepArgument])
        -> ::std::result::Result<(), ::glue::ExecutionError>
    {
        let result = (self.step_fn)(&mut scenario.glue_scenario, args);
        result

    }

    /// The step definition pattern for error reporting only.
    pub fn get_pattern(&self) -> &String {
        unimplemented!();
    }
}
