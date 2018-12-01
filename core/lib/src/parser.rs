use error::Result;
use gherkin::event::{self, CucumberEvent, PickleEvent};
use std::fs;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn parse_pickle_events<P: AsRef<Path>>(features_dir: P) -> Result<Vec<PickleEvent>> {
    let walk_dir = WalkDir::new(features_dir.as_ref())
        .follow_links(true);

    let mut pickle_events = Vec::new();

    for entry in walk_dir {
        let entry: DirEntry = entry?;
        let path = entry.path();

        if !is_feature_file(&entry) {
            continue;
        }

        let feature = fs::read_to_string(path)?;
        let uri = path.to_string_lossy().to_owned();

        let cucumber_events = event::generate(feature, uri)?;
        pickle_events.reserve_exact(cucumber_events.len());

        for cucumber_event in cucumber_events {
            if let CucumberEvent::Pickle(pickle_event) = cucumber_event {
                pickle_events.push(pickle_event);
            }
        }
    }

    Ok(pickle_events)
}

fn is_feature_file(entry: &DirEntry) -> bool {
    entry.file_name().to_string_lossy().ends_with(".feature")
}
