#![feature(proc_macro_hygiene)]
#![warn(rust_2018_idioms)]

use std::path::PathBuf;

use cuke_runner::{Config, ExecutionMode, Glue};
use cuke_runner_listener::{ProgressBarListener, ProgressStyle, JsonReportListener};

mod steps;

#[test]
fn test_cucumber_features() {
    let glue = cuke_runner::glue![steps];

    let output_dir = &[
        env!("CARGO_MANIFEST_DIR"),
        "target",
        "cucumber",
    ].iter().collect::<PathBuf>();

    std::fs::create_dir_all(&output_dir).unwrap();
    let json_report_path = output_dir.join("report.json");
    let mut json_report_file = std::fs::File::create(json_report_path).unwrap();

    let config = Config {
        features_dir: &[env!("CARGO_MANIFEST_DIR"), "tests", "features"].iter().collect::<PathBuf>(),
        output_dir,
        strict: true,
        colored_output: true,
        dry_run: false,
        tags: &[],
        execution_mode: ExecutionMode::ParallelScenarios {
            event_listeners: &[
                &ProgressBarListener::with_style(ProgressStyle::default_bar()
                    .template("[{elapsed}] [{bar:60.cyan/blue}] {pos}/{len}")
                    .progress_chars("=> ")),
                &JsonReportListener::with_writer(&mut json_report_file),
            ],
        },
    };

    cuke_runner::execute_cucumber_tests(glue, config);
}
