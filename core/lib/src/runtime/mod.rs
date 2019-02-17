use std::time::SystemTime;
use std::fs;
use std::path::PathBuf;
use {Config, ExecutionMode};
use runner::{EventBus, SyncEventBus, EventPublisher, Runner};
use self::event_listener::{TestSummaryListener, SyncTestSummaryListener, ExitStatusListener, SyncExitStatusListener};
use crate::api::event::{Event, EventListener, SyncEventListener};
use walkdir::{DirEntry, WalkDir};
use rayon::prelude::*;
pub use self::glue::*;
pub use self::hook_definition::*;
pub use self::scenario::*;
pub use self::step_definition::*;
pub use self::step_definition_match::*;
pub use self::step_expression::*;
pub use self::test_case::*;

mod glue;
mod step_definition;
mod hook_definition;
mod step_expression;
pub mod test_case;
mod scenario;
mod step_definition_match;
pub mod event_listener;


pub fn run(glue: Glue, config: Config) -> i32 {
    let runner = Runner::new(glue, config.dry_run);

    match config.execution_mode {
        ExecutionMode::Sequential { event_listeners } => {
            let exit_status_listener = ExitStatusListener::new();
            let test_summary_listener = TestSummaryListener::new();

            let mut listeners: Vec<&EventListener> = Vec::with_capacity(2 + event_listeners.len());
            listeners.push(&exit_status_listener);
            listeners.push(&test_summary_listener);

            for event_listener in event_listeners {
                listeners.push(*event_listener);
            }

            let event_bus = EventBus::new(listeners);

            event_bus.send(Event::TestRunStarted {
                time: SystemTime::now(),
            });

            run_sequential(runner, &event_bus, &config);

            event_bus.send(Event::TestRunFinished {
                time: SystemTime::now(),
            });

            test_summary_listener.print_test_summary();
            exit_status_listener.get_exit_status(config.strict)
        },
        ExecutionMode::ParallelFeatures { event_listeners } => {
            init_rayon();

            let exit_status_listener = SyncExitStatusListener::new();
            let test_summary_listener = SyncTestSummaryListener::new();

            let mut listeners: Vec<&SyncEventListener> = Vec::with_capacity(2 + event_listeners.len());
            listeners.push(&exit_status_listener);
            listeners.push(&test_summary_listener);

            for event_listener in event_listeners {
                listeners.push(*event_listener);
            }

            let event_bus = SyncEventBus::new(listeners);

            event_bus.send(Event::TestRunStarted {
                time: SystemTime::now(),
            });

            run_parallel_features(runner, &event_bus, &config);

            event_bus.send(Event::TestRunFinished {
                time: SystemTime::now(),
            });

            test_summary_listener.print_test_summary();
            exit_status_listener.get_exit_status(config.strict)
        },
        ExecutionMode::ParallelScenarios { event_listeners } => {
            init_rayon();

            let exit_status_listener = SyncExitStatusListener::new();
            let test_summary_listener = SyncTestSummaryListener::new();

            let mut listeners: Vec<&SyncEventListener> = Vec::with_capacity(2 + event_listeners.len());
            listeners.push(&exit_status_listener);
            listeners.push(&test_summary_listener);

            for event_listener in event_listeners {
                listeners.push(*event_listener);
            }

            let event_bus = SyncEventBus::new(listeners);

            event_bus.send(Event::TestRunStarted {
                time: SystemTime::now(),
            });

            run_parallel_scenarios(runner, &event_bus, &config);

            event_bus.send(Event::TestRunFinished {
                time: SystemTime::now(),
            });

            test_summary_listener.print_test_summary();
            exit_status_listener.get_exit_status(config.strict)
        },
    }
}

fn run_sequential(runner: Runner, event_bus: &EventBus, config: &Config) {
    let walk_dir = WalkDir::new(config.features_dir)
        .follow_links(true);

    let mut gherkin_parser = gherkin::Parser::default();
    let mut gherkin_compiler = gherkin::cuke::Compiler::default();

    for dir_entry_result in walk_dir {
        let entry = dir_entry_result.unwrap();

        if !entry.file_name().to_string_lossy().ends_with(".feature") {
            continue;
        }

        let path = entry.into_path();
        let source = match fs::read_to_string(&path) {
            Ok(source) => source,
            Err(err) => panic!("could not read feature file \"{}\": {}", &path.display(), err),
        };

        let gherkin_document = match gherkin_parser.parse_str(&source) {
            Ok(document) => document,
            Err(err) => panic!("could not parse feature file \"{}\": {}", &path.display(), err),
        };

        let uri = path.display().to_string();

        let feature = match gherkin_document.feature {
            Some(ref feature) => feature,
            None => continue,
        };

        event_bus.send(Event::TestSourceRead {
            time: SystemTime::now(),
            uri: &uri,
            source: &source,
            feature: &feature,
        });

        for cuke in gherkin_compiler.compile(&gherkin_document) {
            runner.run(&uri, cuke, event_bus);
        }
    }
}

fn init_rayon() {
    rayon::ThreadPoolBuilder::new()
        .thread_name(|thread_index| format!("rayon-{}", thread_index))
        .build_global()
        .expect("Failed to build global rayon thread pool");
}

fn run_parallel_features(runner: Runner, event_bus: &SyncEventBus, config: &Config) {
    let walk_dir = WalkDir::new(config.features_dir)
        .follow_links(true);

    let feature_file_paths = walk_dir.into_iter()
        .map(Result::unwrap)
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".feature"))
        .map(DirEntry::into_path)
        .collect::<Vec<PathBuf>>();

    feature_file_paths.into_par_iter().for_each(|path| {
        let source = match fs::read_to_string(&path) {
            Ok(source) => source,
            Err(err) => panic!("could not read feature file \"{}\": {}", &path.display(), err),
        };

        let mut gherkin_parser = gherkin::Parser::default();
        let gherkin_document = match gherkin_parser.parse_str(&source) {
            Ok(document) => document,
            Err(err) => panic!("could not parse feature file \"{}\": {}", &path.display(), err),
        };

        let uri = path.display().to_string();

        let feature = match gherkin_document.feature {
            Some(ref feature) => feature,
            None => return,
        };

        event_bus.send(Event::TestSourceRead {
            time: SystemTime::now(),
            uri: &uri,
            source: &source,
            feature: &feature,
        });

        let mut gherkin_compiler = gherkin::cuke::Compiler::default();
        for cuke in gherkin_compiler.compile(&gherkin_document) {
            runner.run(&uri, cuke, event_bus);
        }
    });
}

fn run_parallel_scenarios(runner: Runner, event_bus: &SyncEventBus, config: &Config) {
    let walk_dir = WalkDir::new(config.features_dir)
        .follow_links(true);

    let mut gherkin_parser = gherkin::Parser::default();
    let mut gherkin_compiler = gherkin::cuke::Compiler::default();

    let gherkin_documents = walk_dir.into_iter()
        .map(Result::unwrap)
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".feature"))
        .map(DirEntry::into_path)
        .filter_map(|path| {
            let source = match fs::read_to_string(&path) {
                Ok(source) => source,
                Err(err) => panic!("could not read feature file \"{}\": {}", &path.display(), err),
            };

            let gherkin_document = match gherkin_parser.parse_str(&source) {
                Ok(document) => document,
                Err(err) => panic!("could not parse feature file \"{}\": {}", &path.display(), err),
            };

            let uri = path.display().to_string();

            let feature = match gherkin_document.feature {
                Some(ref feature) => feature,
                None => return None,
            };

            event_bus.send(Event::TestSourceRead {
                time: SystemTime::now(),
                uri: &uri,
                source: &source,
                feature: &feature,
            });

            Some((uri, gherkin_document))
        })
        .collect::<Vec<_>>();

    let cukes = gherkin_documents.iter()
        .flat_map(|(uri, gherkin_document)| {
            gherkin_compiler.compile(gherkin_document).into_iter()
                .map(|cuke| (uri, cuke))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    cukes.into_par_iter().for_each(|(uri, cuke)| runner.run(&uri, cuke, event_bus));
}
