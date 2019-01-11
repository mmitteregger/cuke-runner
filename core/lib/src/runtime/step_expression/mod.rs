use regex::Regex;

use glue::{StepArgument, ExpressionArgument};

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

    pub fn matched_arguments(&self, text: &str) -> Option<Vec<StepArgument>> {
        let mut matches = self.regex.find_iter(text);

        let mut caps = self.regex.captures(text)?;

        let matched_arguments = caps.iter()
            .skip(1) // The first match always corresponds to the overall match of the regex.
            .filter_map(|opt_mat| {
                opt_mat.map(|mat| StepArgument::Expression(ExpressionArgument::from(mat)))
            })
            .collect::<Vec<StepArgument>>();

        Some(matched_arguments)
    }
}
