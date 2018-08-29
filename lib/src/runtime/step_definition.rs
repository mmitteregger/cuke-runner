//use std::time::Duration;
use std::any::{TypeId, Any};
use std::fmt;

use gherkin::pickle::PickleStep;

use api::SourceCodeLocation;
use codegen::StepHandler;
use super::step_expression::{StepExpression, Argument};

#[derive(Clone)]
pub struct StepDefinition {
    pub expression: StepExpression,
    pub parameter_infos: Vec<TypeId>,
//    pub timeout: Duration,
    pub handler: StepHandler,
    pub location: SourceCodeLocation,
}

impl fmt::Debug for StepDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        f.debug_struct("StepDefinition")
            .field("expression", &self.expression)
            .field("parameter_infos", &self.parameter_infos)
//            .field("timeout", &self.timeout)
            .field("handler", &"<step_handler_function>")
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
    pub fn matched_arguments(&self, step: &PickleStep) -> Option<Vec<Box<Argument>>> {
        unimplemented!();
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
    pub fn execute(&self, language: String, args: Vec<Box<Any>>) {
        unimplemented!();
    }

    /// The step definition pattern for error reporting only.
    pub fn get_pattern(&self) -> &String {
        unimplemented!();
    }
}
