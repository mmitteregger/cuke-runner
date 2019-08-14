#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate cuke_runner;
extern crate cuke_runner_listener;

use std::path::PathBuf;

use cuke_runner::{Config, ExecutionMode, Glue};
use cuke_runner_listener::{ProgressBarListener, ProgressStyle};

mod steps;

#[test]
fn test_cucumber_features() {
    let glue = glue![steps];

    let config = Config {
        features_dir: &[env!("CARGO_MANIFEST_DIR"), "tests", "features"].iter().collect::<PathBuf>(),
        output_dir: &[env!("CARGO_MANIFEST_DIR"), "target", "cucumber"].iter().collect::<PathBuf>(),
        strict: true,
        colored_output: true,
        dry_run: false,
        tags: vec![],
        execution_mode: ExecutionMode::ParallelScenarios {
            event_listeners: &[
                &ProgressBarListener::with_style(ProgressStyle::default_bar()
                    .template("[{elapsed}] [{bar:60.cyan/blue}] {pos}/{len}")
                    .progress_chars("=> ")),
            ],
        },
    };

    cuke_runner::execute_cucumber_tests(glue, config);
}
