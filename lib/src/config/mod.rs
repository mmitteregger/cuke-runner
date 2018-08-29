use std::path::PathBuf;
use std::default::Default;

pub use self::error::Error;

mod error;

#[derive(Debug)]
pub struct Config {
    pub features_dir: PathBuf,
    pub output_dir: PathBuf,
    pub strict: bool,
    pub colored_output: bool,
    pub dry_run: bool,
    pub tags: Vec<String>,
    pub execution_mode: ExecutionMode,
}

/// Controls how the cucumber tests are executed.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum ExecutionMode {
    /// Execute all scenarios in parallel.
    ParallelScenarios,
    /// Execute all features in parallel, but not scenarios from the same feature.
    ParallelFeatures,
    /// Execute every scenario one after another (no parallelism).
    Sequential,
}

impl Default for ExecutionMode {
    fn default() -> ExecutionMode {
        ExecutionMode::ParallelScenarios
    }
}
