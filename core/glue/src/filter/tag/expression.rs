use gherkin::cuke::Tag;

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(LiteralExpression),
    Or(OrExpression),
    And(AndExpression),
    Not(NotExpression),
    True,
}

impl Expression {
    pub fn evaluate(&self, tags: &[Tag]) -> bool {
        use self::Expression::*;

        match self {
            Literal(expression) => expression.evaluate(tags),
            Or(expression) => expression.evaluate(tags),
            And(expression) => expression.evaluate(tags),
            Not(expression) => expression.evaluate(tags),
            True => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LiteralExpression {
    pub value: String,
}

impl LiteralExpression {
    pub fn evaluate(&self, tags: &[Tag]) -> bool {
        tags.iter().any(|tag| tag.name == self.value)
    }
}

#[derive(Debug, Clone)]
pub struct OrExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl OrExpression {
    pub fn evaluate(&self, tags: &[Tag]) -> bool {
        self.left.evaluate(tags) || self.right.evaluate(tags)
    }
}

#[derive(Debug, Clone)]
pub struct AndExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl AndExpression {
    pub fn evaluate(&self, tags: &[Tag]) -> bool {
        self.left.evaluate(tags) && self.right.evaluate(tags)
    }
}

#[derive(Debug, Clone)]
pub struct NotExpression {
    pub expression: Box<Expression>,
}

impl NotExpression {
    pub fn evaluate(&self, tags: &[Tag]) -> bool {
        !self.expression.evaluate(tags)
    }
}
