use std::fs::File;
use std::path::Path;

use gherkin::Parser;
use gherkin::ast::*;
use walkdir::{WalkDir, DirEntry};

use error::Result;

pub fn parse_gherkin_documents<P: AsRef<Path>>(features_dir: P) -> Result<Vec<GherkinDocument>> {
    let walk_dir = WalkDir::new(features_dir.as_ref())
        .follow_links(true);

    let mut parser = Parser::default();
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
