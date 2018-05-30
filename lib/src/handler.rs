use data::StepData;

/// The type of a step handler.
pub type StepHandler = fn(&StepData) -> ::error::Result<()>;
