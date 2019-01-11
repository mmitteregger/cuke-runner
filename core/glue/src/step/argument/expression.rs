#[derive(Debug, Clone)]
pub struct Expression {
    value: String,
    start: usize,
    end: usize,
}

impl Expression {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn start(&self) -> usize {
        self.start
    }
    pub fn end(&self) -> usize {
        self.end
    }
}

impl<'t> From<regex::Match<'t>> for Expression {
    fn from(mat: regex::Match<'t>) -> Self {
        Expression {
            value: mat.as_str().to_owned(),
            start: mat.start(),
            end: mat.end(),
        }
    }
}
