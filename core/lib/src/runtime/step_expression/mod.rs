use regex::Regex;

pub use self::argument::*;

mod argument;

#[derive(Debug, Clone)]
pub struct StepExpression {
    pub regex: Regex,
}

impl StepExpression {
    pub fn from_regex(regex: &str) -> StepExpression {
        StepExpression {
            regex: Regex::new(regex).unwrap(),
        }
    }

    pub fn matched_arguments(&self, text: &str) -> Option<Vec<Argument>> {
        let mut matches = self.regex.find_iter(text);

        // First match is the whole text, if any
        if let None = matches.next() {
            return None;
        }

        let matched_arguments = matches.into_iter()
            .map(|mat| Argument::Expression(mat.as_str().to_string()))
            .collect::<Vec<Argument>>();
        Some(matched_arguments)
    }
}
