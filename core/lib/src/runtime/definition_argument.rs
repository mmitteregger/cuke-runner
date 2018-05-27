use api;
use runtime::{step_expression, Argument};

pub struct DefinitionArgument<'a> {
    expression_argument: &'a Argument,
}

impl<'a> DefinitionArgument<'a> {
    pub fn new(expression_argument: &'a Argument) -> DefinitionArgument<'a> {
        DefinitionArgument { expression_argument }
    }

    pub fn create_arguments<A: api::Argument + Sized>(arg_matches: &Vec<step_expression::Argument>) -> &Vec<A> {
        unimplemented!();
//        arg_matches.iter()
//            .filter_map(|arg_match| {
//                arg_match.as_any().downcast_ref::<ExpressionArgument>()
//            })
//            .map(DefinitionArgument::new)
//            .map(Box::new)
//            .map(|definition_argument| definition_argument as Box<api::Argument>)
//            .collect::<Vec<_>>()
    }
}

impl<'a> api::Argument for DefinitionArgument<'a> {
    fn get_value(&self) -> String {
        unimplemented!()
    }

    fn get_start(&self) -> u32 {
        unimplemented!()
    }

    fn get_end(&self) -> u32 {
        unimplemented!()
    }
}
