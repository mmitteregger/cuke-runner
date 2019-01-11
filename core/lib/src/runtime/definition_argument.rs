use api;
use glue::step::argument::{StepArgument, Expression};

pub struct DefinitionArgument {
    expression: Expression,
}

impl DefinitionArgument {
    pub fn new(expression: Expression) -> DefinitionArgument {
        DefinitionArgument {
            expression,
        }
    }

    pub fn create_arguments(arg_matches: &[StepArgument]) -> Vec<Box<api::Argument>> {
        arg_matches.iter()
            .filter_map(|arg_match| {
                match arg_match {
                    StepArgument::Expression(expression) => Some(expression),
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
        Some(self.expression.value())
    }

    fn start(&self) -> usize {
        self.expression.start()
    }

    fn end(&self) -> usize {
        self.expression.end()
    }
}
