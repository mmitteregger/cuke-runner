use error::Result;
use gherkin::event::{self, CucumberEvent, PickleEvent};
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use std::rc::Rc;
use walkdir::{DirEntry, WalkDir};
use runner::EventBus;
use api::event::Event;

pub fn parse_pickle_events<P: AsRef<Path>>(features_dir: P, event_bus: &EventBus)
    -> Result<Vec<PickleEvent>>
{
    let walk_dir = WalkDir::new(features_dir.as_ref())
        .follow_links(true);

    let mut pickle_events = Vec::new();

    for entry in walk_dir {
        let entry: DirEntry = entry?;
        let path = entry.path();

        if !is_feature_file(&entry) {
            continue;
        }

        let feature_file_content = fs::read_to_string(path)?;
        let uri = path.to_string_lossy().to_owned();

        let cucumber_events = event::generate(feature_file_content.as_ref(), uri)?;
        pickle_events.reserve_exact(cucumber_events.len());

        for cucumber_event in cucumber_events {
            match cucumber_event {
                CucumberEvent::GherkinDocument(gherkin_document_event) => {
                    if let Some(feature) = gherkin_document_event.document.feature {
                        event_bus.send(Event::TestSourceRead {
                            time: SystemTime::now(),
                            uri: path.to_string_lossy().to_owned().as_ref(),
                            source: &feature_file_content,
                            feature: &Rc::new(feature),
                        });
                    }
                },
                CucumberEvent::Pickle(pickle_event) => pickle_events.push(pickle_event),
                _ => {},
            }
        }
    }

    Ok(pickle_events)
}

fn is_feature_file(entry: &DirEntry) -> bool {
    entry.file_name().to_string_lossy().ends_with(".feature")
}
