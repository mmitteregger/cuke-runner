use regex::Regex;

pub use self::argument::*;

mod argument;

#[derive(Debug)]
pub struct StepExpression {
    pub regex: Regex,
}

impl StepExpression {
    pub fn from_regex(regex: &str) -> StepExpression {
        StepExpression {
            regex: Regex::new(regex).unwrap(),
        }
    }
}
