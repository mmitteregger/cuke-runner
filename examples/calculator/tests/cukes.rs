#![feature(proc_macro_hygiene)]

extern crate calculator;
#[macro_use]
extern crate cuke_runner;

use std::path::PathBuf;
use cuke_runner::{Config, ExecutionMode, Glue};
use cuke_runner::event_listener::PrettyFormatter;

mod steps;

#[test]
fn test_cucumber_features_sequential() {
    let glue = glue![steps];

    let config = Config {
        features_dir: &[env!("CARGO_MANIFEST_DIR"), "tests", "features"].iter().collect::<PathBuf>(),
        output_dir: &[env!("CARGO_MANIFEST_DIR"), "target", "cucumber"].iter().collect::<PathBuf>(),
        strict: true,
        colored_output: true,
        dry_run: false,
        tags: vec![],
        execution_mode: ExecutionMode::Sequential {
            event_listeners: &[
                &PrettyFormatter::new(),
            ],
        },
    };

    cuke_runner::execute_cucumber_tests(glue, config);
}
