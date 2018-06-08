use std::any::Any;
use std::fmt::Debug;

pub trait Argument: Debug + Send + Sync + Any {
    fn as_any(&self) -> &Any;

    fn get_value(&self) -> Box<Any>;
}

#[derive(Debug)]
pub struct ExpressionArgument;

#[derive(Debug)]
pub struct DocStringArgument;

#[derive(Debug)]
pub struct DataTableArgument;

impl Argument for ExpressionArgument {
    fn as_any(&self) -> &Any {
        self
    }

    fn get_value(&self) -> Box<Any> {
        unimplemented!()
    }
}

impl Argument for DocStringArgument {
    fn as_any(&self) -> &Any {
        self
    }

    fn get_value(&self) -> Box<Any> {
        unimplemented!()
    }
}

impl Argument for DataTableArgument {
    fn as_any(&self) -> &Any {
        self
    }

    fn get_value(&self) -> Box<Any> {
        unimplemented!()
    }
}
