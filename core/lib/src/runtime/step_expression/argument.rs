use self::Argument::*;

#[derive(Debug, Clone)]
pub enum Argument {
    Expression(String),
    DocString(String),
    DataTable,
}

impl Argument {
    fn get_value(&self) -> &str {
        match self {
            Expression(value) => value,
            DocString(value) => value,
            DataTable => unimplemented!("get DataTable value"),
        }
    }
}
