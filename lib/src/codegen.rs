use std::path::PathBuf;
use std::env;

use handler::Handler;
use StepKeyword;

pub struct StaticStepInfo {
    pub name: &'static str,
    pub keyword: StepKeyword,
    pub text: &'static str,
    pub handler: Handler,
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
