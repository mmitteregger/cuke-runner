use std::time::SystemTime;
use std::mem;
use {Config, ExecutionMode};
use parser;
use runner::{EventBus, Runner};
use self::event_listener::*;
use crate::api::event::Event;
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
    let feature_pickles_map = parser::parse_pickle_events(&config.features_dir, &event_bus).unwrap();

    for (feature_file, pickle_events) in feature_pickles_map {
        for pickle_event in pickle_events {
            runner.run_pickle(&feature_file, pickle_event, &event_bus);
        }
    }
}

fn run_parallel_features(_runner: Runner, _event_bus: &mut EventBus, _config: &Config) {
    unimplemented!("run_parallel_features");
//    let pickle_events = parser::parse_pickle_events(&config.features_dir, &event_bus).unwrap();
//    let mut pickle_events_per_feature = HashMap::new();
//
//    for pickle_event in pickle_events {
//        pickle_events_per_feature.entry(pickle_event.uri.clone())
//            .or_insert_with(Vec::new)
//            .push(pickle_event);
//    }
//
//    for (feature_uri, pickle_events) in pickle_events_per_feature {
//        pickle_events.into_par_iter()
//            .for_each(|pickle_event| runner.run_pickle(pickle_event, &event_bus));
//    }
}

fn run_parallel_scenarios(_runner: Runner, _event_bus: &mut EventBus, _config: &Config) {
    unimplemented!("run_parallel_scenarios");
//    let _pickle_events = parser::parse_pickle_events(&config.features_dir, &event_bus).unwrap();
//
////    pickle_events.into_par_iter()
////        .for_each(|pickle_event| runner.run_pickle(pickle_event, &event_bus));
}
