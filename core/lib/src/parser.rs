use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

use gherkin::event::{self, CucumberEvent, PickleEvent};
use walkdir::{DirEntry, WalkDir};

use api::FeatureFile;
use api::event::Event;
use error::Result;
use runner::EventBus;

pub(crate) fn parse_pickle_events<P: AsRef<Path>>(features_dir: P, event_bus: &EventBus)
    -> Result<HashMap<FeatureFile, Vec<PickleEvent>>>
{
    let walk_dir = WalkDir::new(features_dir.as_ref())
        .follow_links(true);

    let mut pickle_events = HashMap::new();

    for entry in walk_dir {
        let entry: DirEntry = entry?;
        let path = entry.path();

        if !is_feature_file(&entry) {
            continue;
        }

        let feature_file_content = fs::read_to_string(path)?;
        let uri = path.to_string_lossy().to_owned();

        let cucumber_events = event::generate(feature_file_content.as_ref(), uri)?;
        let capacity = cucumber_events.len() - 1;
        let mut events: Option<&mut Vec<PickleEvent>> = None;

        for cucumber_event in cucumber_events {
            match cucumber_event {
                CucumberEvent::GherkinDocument(gherkin_document_event) => {
                    if let Some(feature) = gherkin_document_event.document.feature {
                        let feature_file = FeatureFile {
                            uri: path.to_string_lossy().to_string(),
                            feature,
                        };

                        event_bus.send(Event::TestSourceRead {
                            time: SystemTime::now(),
                            uri: path.to_string_lossy().to_owned().as_ref(),
                            source: &feature_file_content,
                            feature_file: &feature_file,
                        });

                        match pickle_events.entry(feature_file) {
                            Entry::Occupied(_entry) => panic!("duplicate feature file"),
                            Entry::Vacant(entry) => {
                                events = Some(entry.insert(Vec::with_capacity(capacity)));
                            },
                        };
                    }
                },
                CucumberEvent::Pickle(pickle_event) => {
                    events.as_mut().unwrap().push(pickle_event);
                },
                _ => {},
            }
        }
    }

    Ok(pickle_events)
}

fn is_feature_file(entry: &DirEntry) -> bool {
    entry.file_name().to_string_lossy().ends_with(".feature")
}
