use std::path::Path;
use std::default::Default;

pub use self::error::Error;
use api::event::EventListener;

mod error;

#[derive(Debug)]
pub struct Config<'c> {
    pub features_dir: &'c Path,
    pub output_dir: &'c Path,
    pub strict: bool,
    pub colored_output: bool,
    pub dry_run: bool,
    pub tags: Vec<String>,
    pub execution_mode: ExecutionMode,
    pub event_listeners: &'c mut [&'c mut dyn EventListener],
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
