/*!
Cucumber for Rust with a focus on ease-of-use.
*/

extern crate gherkin;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate walkdir;

use std::fs::File;
use std::path::Path;

use gherkin::ast::GherkinDocument;
use walkdir::{WalkDir, DirEntry};

pub use state::State;
pub use step_keyword::StepKeyword;

use config::CukeConfig;
use error::Result;

mod config;
mod error;
#[doc(hidden)]
pub mod macros;
mod state;
mod step_keyword;

pub fn run_cukes<P: AsRef<Path>>(tests_base_path: P) {
    match run(tests_base_path) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("Uh oh, looks like some cukes have rotten: {}", error);
            ::std::process::exit(-1);
        },
    };
}

fn run<P: AsRef<Path>>(tests_base_path: P) -> Result<()> {
    let config = CukeConfig::read(tests_base_path)?;
    // TODO: Use gherkin_documents
    let _gherkin_documents = parse_gherkin_documents(&config.features)?;

    Ok(())
}

fn parse_gherkin_documents<P: AsRef<Path>>(features_dir: P) -> Result<Vec<GherkinDocument>> {
    let walk_dir = WalkDir::new(features_dir.as_ref())
        .follow_links(true);

    let mut parser = gherkin::Parser::default();
    let mut gherkin_documents = Vec::new();

    for entry in walk_dir {
        let entry: DirEntry = entry?;
        let path = entry.path();

        if !is_feature_file(&entry) {
            continue;
        }

        let file = File::open(path)?;
        let gherkin_document = parser.parse_reader(&file)?;
        gherkin_documents.push(gherkin_document);
    }

    Ok(gherkin_documents)
}

fn is_feature_file(entry: &DirEntry) -> bool {
    entry.file_name().to_string_lossy().ends_with(".feature")
}
