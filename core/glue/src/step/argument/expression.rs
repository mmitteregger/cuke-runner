/// The lifetime parameter `'s` refers to the lifetime of the step.
/// It cannot escape the step function.
#[derive(Debug, Clone)]
pub struct Expression<'s> {
    value: &'s str,
    start: usize,
    end: usize,
}

impl<'s> Expression<'s> {
    pub fn value(&'s self) -> &'s str {
        &self.value
    }
    pub fn start(&self) -> usize {
        self.start
    }
    pub fn end(&self) -> usize {
        self.end
    }
}

impl<'s, 't: 's> From<regex::Match<'t>> for Expression<'s> {
    fn from(mat: regex::Match<'t>) -> Self {
        Expression {
            value: mat.as_str(),
            start: mat.start(),
            end: mat.end(),
        }
    }
}
