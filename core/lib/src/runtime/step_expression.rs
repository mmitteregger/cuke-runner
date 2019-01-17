use regex::Regex;

use glue::step::argument::{StepArgument, Expression};

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

    pub fn matched_arguments<'s>(&'s self, text: &'s str) -> Option<Vec<StepArgument<'s>>> {
        let caps = self.regex.captures(text)?;

        let matched_arguments = caps.iter()
            .skip(1) // The first match always corresponds to the overall match of the regex.
            .filter_map(|opt_mat| {
                opt_mat.map(|mat| StepArgument::Expression(Expression::from(mat)))
            })
            .collect::<Vec<StepArgument>>();

        Some(matched_arguments)
    }
}
