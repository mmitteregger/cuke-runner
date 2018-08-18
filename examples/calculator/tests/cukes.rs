#![feature(plugin, decl_macro)]
#![plugin(cuke_runner_plugin)]

extern crate calculator;
#[macro_use]
extern crate cuke_runner;
extern crate cuke_runner_derive;

use std::path::PathBuf;

use cuke_runner::{Config, ExecutionMode, Glue};

mod steps;

#[test]
fn test_cucumber_features() {
    let glue = Glue::from(steps::STEP_DEFINITIONS);
    let config = Config {
        features_dir: [env!("CARGO_MANIFEST_DIR"), "tests", "features"].iter().collect(),
        output_dir: [env!("CARGO_MANIFEST_DIR"), "target", "cucumber"].iter().collect(),
        strict: true,
        colored_output: true,
        dry_run: false,
        tags: Vec::new(),
        execution_mode: ExecutionMode::Sequential,
    };

    cuke_runner::execute_cucumber_tests(glue, config);
}
