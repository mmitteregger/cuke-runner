use glue::{StepArgument, ExpressionArgument};

use api;

pub struct DefinitionArgument {
    expression_argument: ExpressionArgument,
}

impl DefinitionArgument {
    pub fn new(expression_argument: ExpressionArgument) -> DefinitionArgument {
        DefinitionArgument { expression_argument }
    }

    pub fn create_arguments(arg_matches: &[StepArgument]) -> Vec<Box<api::Argument>> {
        arg_matches.iter()
            .filter_map(|arg_match| {
                match arg_match {
                    StepArgument::Expression(expression_argument) => Some(expression_argument),
                    _ => None,
                }
            })
            .map(Clone::clone)
            .map(DefinitionArgument::new)
            .map(Box::new)
            .map(|definition_argument| definition_argument as Box<api::Argument>)
            .collect::<Vec<_>>()
    }
}

impl api::Argument for DefinitionArgument {
    fn value(&self) -> Option<&str> {
        Some(self.expression_argument.value())
    }

    fn start(&self) -> usize {
        self.expression_argument.start()
    }

    fn end(&self) -> usize {
        self.expression_argument.end()
    }
}
