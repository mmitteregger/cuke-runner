use std::time::SystemTime;
use std::mem;
use std::fs;
use {Config, ExecutionMode};
use runner::{EventBus, Runner};
use self::event_listener::*;
use crate::api::event::Event;
use walkdir::WalkDir;
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
mod event_listener;


pub fn run(glue: Glue, mut config: Config) -> i32 {
    let mut exit_status_listener = ExitStatusListener::new();
    let mut pretty_formatter = PrettyFormatter::new();
    let mut test_summary_listener = TestSummaryListener::new();

    let mut event_bus = EventBus::new();
    event_bus.register_listener(&mut exit_status_listener);
    event_bus.register_listener(&mut pretty_formatter);
    event_bus.register_listener(&mut test_summary_listener);

    let additional_event_listeners =
        mem::replace(&mut config.event_listeners, &mut []);
    for event_listener in additional_event_listeners.iter_mut() {
        event_bus.register_listener(*event_listener);
    }

    let runner = Runner::new(glue, config.dry_run);

    event_bus.send(Event::TestRunStarted {
        time: SystemTime::now(),
    });

    match config.execution_mode {
        ExecutionMode::Sequential => run_sequential(runner, &mut event_bus, &config),
        ExecutionMode::ParallelFeatures => run_parallel_features(runner, &mut event_bus, &config),
        ExecutionMode::ParallelScenarios => run_parallel_scenarios(runner, &mut event_bus, &config),
    };

    event_bus.send(Event::TestRunFinished {
        time: SystemTime::now(),
    });

    exit_status_listener.get_exit_status(config.strict)
}

fn run_sequential(runner: Runner, event_bus: &mut EventBus, config: &Config) {
    let walk_dir = WalkDir::new(config.features_dir)
        .follow_links(true);

    let mut gherkin_parser = gherkin::Parser::default();
    let mut gherkin_compiler = gherkin::cuke::Compiler::default();

    for dir_entry_result in walk_dir {
        let entry = dir_entry_result.unwrap();

        if !entry.file_name().to_string_lossy().ends_with(".feature") {
            continue;
        }

        let path = entry.path();
        let source = match fs::read_to_string(path) {
            Ok(source) => source,
            Err(err) => panic!("could not read feature file \"{}\": {}", path.display(), err),
        };

        let gherkin_document = match gherkin_parser.parse_str(&source) {
            Ok(document) => document,
            Err(err) => panic!("could not parse feature file \"{}\": {}", path.display(), err),
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
            runner.run(&uri, cuke, &event_bus);
        }
    }
}

fn run_parallel_features(_runner: Runner, _event_bus: &mut EventBus, _config: &Config) {
    unimplemented!("run_parallel_features");
}

fn run_parallel_scenarios(_runner: Runner, _event_bus: &mut EventBus, _config: &Config) {
    unimplemented!("run_parallel_scenarios");
}
