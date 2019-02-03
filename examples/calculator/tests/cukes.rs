#![feature(proc_macro_hygiene, decl_macro)]

extern crate calculator;
#[macro_use]
extern crate cuke_runner;

use std::path::PathBuf;
use cuke_runner::{Config, ExecutionMode, Glue};

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
        execution_mode: ExecutionMode::Sequential,
        event_listeners: &mut [],
    };

    cuke_runner::execute_cucumber_tests(glue, config);
}
