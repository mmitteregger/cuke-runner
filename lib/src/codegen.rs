use std::path::PathBuf;
use std::env;

use data::{StepData, StepKeyword};

/// The type of a step handler.
pub type StepHandler = fn(&StepData) -> ::error::Result<()>;

/// Generated info for a step definition (a `#[step(...)]` annotated function).
pub struct StaticStepDefInfo {
    /// Name of the step definition function.
    pub name: &'static str,
    /// Step definition keyword like "Given", "When" and "Then".
    pub keyword: StepKeyword,
    /// The step definition text to match a step in a cucumber scenario.
    pub text: &'static str,
    /// The generated handler responsible for calling the step definition function.
    pub handler: StepHandler,
}

pub fn __cuke_runner_project_dir() -> PathBuf {
    env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .or_else(|_| {
            match env::current_dir() {
                Ok(cwd) => {
                    let cargo_toml = cwd.join("Cargo.toml");
                    if cargo_toml.exists() {
                        return Ok(cwd);
                    } else {
                        panic!("could not find Cargo.toml in current working directory: {}",
                            cwd.display());
                    }
                }
                Err(error) => return Err(error),
            }
        })
        .expect("could not determine current project directory")
}
