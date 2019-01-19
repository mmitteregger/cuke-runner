use gherkin::pickle::PickleTag;

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(LiteralExpression),
    Or(OrExpression),
    And(AndExpression),
    Not(NotExpression),
    True,
}

impl Expression {
    pub fn evaluate(&self, pickle_tags: &[PickleTag]) -> bool {
        use self::Expression::*;

        match self {
            Literal(expression) => expression.evaluate(pickle_tags),
            Or(expression) => expression.evaluate(pickle_tags),
            And(expression) => expression.evaluate(pickle_tags),
            Not(expression) => expression.evaluate(pickle_tags),
            True => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LiteralExpression {
    pub value: String,
}

impl LiteralExpression {
    pub fn evaluate(&self, pickle_tags: &[PickleTag]) -> bool {
        pickle_tags.iter().any(|tag| tag.name == self.value)
    }
}

#[derive(Debug, Clone)]
pub struct OrExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl OrExpression {
    pub fn evaluate(&self, pickle_tags: &[PickleTag]) -> bool {
        self.left.evaluate(pickle_tags) || self.right.evaluate(pickle_tags)
    }
}

#[derive(Debug, Clone)]
pub struct AndExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl AndExpression {
    pub fn evaluate(&self, pickle_tags: &[PickleTag]) -> bool {
        self.left.evaluate(pickle_tags) && self.right.evaluate(pickle_tags)
    }
}

#[derive(Debug, Clone)]
pub struct NotExpression {
    pub expression: Box<Expression>,
}

impl NotExpression {
    pub fn evaluate(&self, pickle_tags: &[PickleTag]) -> bool {
        !self.expression.evaluate(pickle_tags)
    }
}
