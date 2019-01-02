use api;
use runtime::{step_expression, Argument};

pub struct DefinitionArgument {
    expression_argument: Argument,
}

impl DefinitionArgument {
    pub fn new(expression_argument: Argument) -> DefinitionArgument {
        DefinitionArgument { expression_argument }
    }

    pub fn create_arguments(arg_matches: &Vec<step_expression::Argument>) -> Vec<Box<api::Argument>> {
        arg_matches.iter()
            .filter(|arg_match| {
                match arg_match {
                    Argument::Expression(..) => true,
                    _ => false,
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
    fn value(&self) -> Option<String> {
        unimplemented!()
    }

    fn start(&self) -> usize {
        unimplemented!()
    }

    fn end(&self) -> usize {
        unimplemented!()
    }
}
