use std::any::Any;

use api;
use runtime::step_expression::{self, ExpressionArgument};

pub struct DefinitionArgument<'a> {
    expression_argument: &'a ExpressionArgument,
}

impl<'a> DefinitionArgument<'a> {
    pub fn new(expression_argument: &'a ExpressionArgument) -> DefinitionArgument<'a> {
        DefinitionArgument { expression_argument }
    }

    pub fn create_arguments(arg_matches: &Vec<Box<step_expression::Argument>>) -> &Vec<Box<api::Argument>> {
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
