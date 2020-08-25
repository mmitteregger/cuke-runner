use std::path::Path;
use std::default::Default;

pub use self::error::Error;
use crate::api::event::{EventListener, SyncEventListener};

mod error;

#[derive(Debug)]
pub struct Config<'c> {
    pub features_dir: &'c Path,
    pub output_dir: &'c Path,
    pub strict: bool,
    pub colored_output: bool,
    pub dry_run: bool,
    pub tags: &'c [&'c str],
    pub execution_mode: ExecutionMode<'c>,
}

/// Controls how the cucumber tests are executed.
#[derive(Debug)]
pub enum ExecutionMode<'c> {
    /// Execute all scenarios in parallel.
    ParallelScenarios {
        event_listeners: &'c [&'c dyn SyncEventListener],
    },
    /// Execute all features in parallel, but not scenarios from the same feature.
    ParallelFeatures {
        event_listeners: &'c [&'c dyn SyncEventListener],
    },
    /// Execute every scenario one after another (no parallelism).
    Sequential {
        event_listeners: &'c [&'c dyn EventListener],
    },
}

impl<'c> Default for ExecutionMode<'c> {
    fn default() -> ExecutionMode<'c> {
        ExecutionMode::ParallelScenarios {
            event_listeners: &[],
        }
    }
}
