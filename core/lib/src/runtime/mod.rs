use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;

use gherkin::ast::GherkinDocument;
use gherkin::{IdGenerator, IncrementingIdGenerator};
use gherkin::cuke::Cuke;
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

use crate::{Config, ExecutionMode};
use crate::runner::{EventBus, EventPublisher, Runner, SyncEventBus};
use crate::runtime::filter::Filters;

use crate::api::event::{Event, EventListener, SyncEventListener};

use self::event_listener::{ExitStatusListener, SyncExitStatusListener, SyncTestSummaryListener, TestSummaryListener};
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
mod filter;

pub fn run(glue: Glue, config: Config<'_>) -> i32 {
    cuke_runner_glue::panic::register_cuke_runner_hook();

    let runner = Runner::new(glue, config.dry_run);
    let filters = Filters::from(&config);

    let exit_status = match config.execution_mode {
        ExecutionMode::Sequential { event_listeners } => {
            let exit_status_listener = ExitStatusListener::new();
            let test_summary_listener = TestSummaryListener::new();

            let mut listeners: Vec<&dyn EventListener> = Vec::with_capacity(2 + event_listeners.len());
            listeners.push(&exit_status_listener);
            listeners.push(&test_summary_listener);

            for event_listener in event_listeners {
                listeners.push(*event_listener);
            }

            let event_bus = EventBus::new(listeners);

            run_sequential(runner, filters, &event_bus, &config);

            test_summary_listener.print_test_summary();
            exit_status_listener.get_exit_status(config.strict)
        }
        ExecutionMode::ParallelFeatures { event_listeners } => {
            init_rayon();

            let exit_status_listener = SyncExitStatusListener::new();
            let test_summary_listener = SyncTestSummaryListener::new();

            let mut listeners: Vec<&dyn SyncEventListener> = Vec::with_capacity(2 + event_listeners.len());
            listeners.push(&exit_status_listener);
            listeners.push(&test_summary_listener);

            for event_listener in event_listeners {
                listeners.push(*event_listener);
            }

            let event_bus = SyncEventBus::new(listeners);

            run_parallel_features(runner, filters, &event_bus, &config);

            test_summary_listener.print_test_summary();
            exit_status_listener.get_exit_status(config.strict)
        }
        ExecutionMode::ParallelScenarios { event_listeners } => {
            init_rayon();

            let exit_status_listener = SyncExitStatusListener::new();
            let test_summary_listener = SyncTestSummaryListener::new();

            let mut listeners: Vec<&dyn SyncEventListener> = Vec::with_capacity(2 + event_listeners.len());
            listeners.push(&exit_status_listener);
            listeners.push(&test_summary_listener);

            for event_listener in event_listeners {
                listeners.push(*event_listener);
            }

            let event_bus = SyncEventBus::new(listeners);

            run_parallel_scenarios(runner, filters, &event_bus, &config);

            test_summary_listener.print_test_summary();
            exit_status_listener.get_exit_status(config.strict)
        }
    };

    exit_status
}

struct ParsedGherkinDocument {
    uri: String,
    source: String,
    document: GherkinDocument,
}

struct ParsedCuke<'d> {
    uri: &'d str,
    cuke: Cuke<'d>,
}

fn run_sequential(runner: Runner, filters: Filters<'_>, event_bus: &EventBus<'_>, config: &Config<'_>) {
    let mut id_generator = IncrementingIdGenerator::new();
    let parsed_gherkin_documents = parse_gherking_documents(config, &mut id_generator);
    let parsed_cukes = parse_cukes(&parsed_gherkin_documents, event_bus, &mut id_generator);

    event_bus.send(Event::TestRunStarted {
        time: SystemTime::now(),
        num_cukes: parsed_cukes.len(),
    });

    for parsed_cuke in parsed_cukes {
        if filters.apply(parsed_cuke.uri, &parsed_cuke.cuke) {
            runner.run(&parsed_cuke.uri, parsed_cuke.cuke, event_bus);
        }
    }

    event_bus.send(Event::TestRunFinished {
        time: SystemTime::now(),
    });
}

fn run_parallel_features(runner: Runner, filters: Filters<'_>, event_bus: &SyncEventBus<'_>, config: &Config<'_>) {
    let mut id_generator = IncrementingIdGenerator::new();
    let parsed_gherkin_documents = parse_gherking_documents(config, &mut id_generator);
    let parsed_cukes = parse_cukes(&parsed_gherkin_documents, event_bus, &mut id_generator);

    event_bus.send(Event::TestRunStarted {
        time: SystemTime::now(),
        num_cukes: parsed_cukes.len(),
    });

    let mut feature_cukes = HashMap::with_capacity(parsed_cukes.len());
    for parsed_cuke in parsed_cukes {
        feature_cukes.entry(parsed_cuke.uri)
            .or_insert_with(Vec::new)
            .push(parsed_cuke.cuke);
    }
    feature_cukes.shrink_to_fit();

    feature_cukes.into_par_iter().for_each(|(uri, cukes)| {
        for cuke in cukes {
            if filters.apply(uri, &cuke) {
                runner.run(uri, cuke, event_bus);
            }
        }
    });

    event_bus.send(Event::TestRunFinished {
        time: SystemTime::now(),
    });
}

fn run_parallel_scenarios(runner: Runner, filters: Filters<'_>, event_bus: &SyncEventBus<'_>, config: &Config<'_>) {
    let mut id_generator = IncrementingIdGenerator::new();
    let parsed_gherkin_documents = parse_gherking_documents(config, &mut id_generator);
    let parsed_cukes = parse_cukes(&parsed_gherkin_documents, event_bus, &mut id_generator);

    event_bus.send(Event::TestRunStarted {
        time: SystemTime::now(),
        num_cukes: parsed_cukes.len(),
    });

    parsed_cukes.into_par_iter().for_each(|parsed_cuke| {
        if filters.apply(parsed_cuke.uri, &parsed_cuke.cuke) {
            runner.run(parsed_cuke.uri, parsed_cuke.cuke, event_bus);
        }
    });

    event_bus.send(Event::TestRunFinished {
        time: SystemTime::now(),
    });
}

fn init_rayon() {
    rayon::ThreadPoolBuilder::new()
        .thread_name(|thread_index| format!("cuke-runner-{}", thread_index))
        .build_global()
        .expect("Failed to build global rayon thread pool");
}

fn parse_gherking_documents(
    config: &Config<'_>,
    id_generator: &mut dyn IdGenerator,
) -> Vec<ParsedGherkinDocument>
{
    let walk_dir = WalkDir::new(config.features_dir)
        .follow_links(true);

    let builder = gherkin::DocumentBuilder::with_id_generator(id_generator);
    let mut gherkin_parser = gherkin::Parser::with_builder(builder);

    walk_dir.into_iter()
        .map(Result::unwrap)
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".feature"))
        .map(DirEntry::into_path)
        .map(|path| {
            let source = match fs::read_to_string(&path) {
                Ok(source) => source,
                Err(err) => panic!("could not read feature file \"{}\": {}", &path.display(), err),
            };

            let gherkin_document = match gherkin_parser.parse_str(&source) {
                Ok(document) => document,
                Err(err) => panic!("could not parse feature file \"{}\": {}", &path.display(), err),
            };

            let path_relative_to_features_dir = path.strip_prefix(config.features_dir).unwrap();
            let uri = path_relative_to_features_dir.display().to_string();

            ParsedGherkinDocument {
                uri,
                source,
                document: gherkin_document,
            }
        })
        .collect::<Vec<ParsedGherkinDocument>>()
}

fn parse_cukes<'d>(
    parsed_gherkin_documents: &'d [ParsedGherkinDocument],
    event_publisher: &dyn EventPublisher,
    id_generator: &mut dyn IdGenerator,
) -> Vec<ParsedCuke<'d>>
{
    let mut gherkin_compiler = gherkin::cuke::Compiler::new(id_generator);

    parsed_gherkin_documents.iter()
        .flat_map(|parsed_gherkin_document| {
            let feature = match parsed_gherkin_document.document.feature {
                Some(ref feature) => feature,
                None => return Vec::new(),
            };

            let cukes = gherkin_compiler.compile(&parsed_gherkin_document.document);

            event_publisher.send(Event::TestSourceRead {
                time: SystemTime::now(),
                uri: &parsed_gherkin_document.uri,
                source: &parsed_gherkin_document.source,
                feature: &feature,
                cukes: &cukes,
            });

            cukes.into_iter()
                .map(|cuke| ParsedCuke {
                    uri: &parsed_gherkin_document.uri,
                    cuke,
                })
                .collect::<Vec<ParsedCuke<'_>>>()
        })
        .collect::<Vec<ParsedCuke<'_>>>()
}
