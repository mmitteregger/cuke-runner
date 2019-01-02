use {Config, ExecutionMode};
use parser;
use runner::{EventBus, Runner};
pub use self::definition_argument::*;
use self::event_listener::*;
pub use self::glue::*;
pub use self::hook_definition::*;
pub use self::scenario::*;
pub use self::step_definition::*;
pub use self::step_definition_match::*;
pub use self::step_expression::*;
pub use self::tag_predicate::*;
pub use self::test_case::*;
use std::collections::HashMap;

mod glue;
mod step_definition;
mod hook_definition;
mod step_expression;
mod tag_predicate;
pub mod test_case;
mod scenario;
mod step_definition_match;
mod definition_argument;
mod event_listener;


pub fn run(glue: Glue, config: Config) -> i32 {
    let mut exit_status_listener = ExitStatusListener::new();
    let mut pretty_formatter = PrettyFormatter::new();
    let mut test_summary_listener = TestSummaryListener::new();

    let mut event_bus = EventBus::new();
    event_bus.register_listener(&mut exit_status_listener);
    event_bus.register_listener(&mut pretty_formatter);
    event_bus.register_listener(&mut test_summary_listener);

    let runner = Runner::new(glue, config.dry_run);

    match config.execution_mode {
        ExecutionMode::Sequential => run_sequential(runner, event_bus, &config),
        ExecutionMode::ParallelFeatures => run_parallel_features(runner, event_bus, &config),
        ExecutionMode::ParallelScenarios => run_parallel_scenarios(runner, event_bus, &config),
    }

    let exit_status = exit_status_listener.get_exit_status(config.strict);
    exit_status
}

fn run_sequential(runner: Runner, event_bus: EventBus, config: &Config) {
    let pickle_events = parser::parse_pickle_events(&config.features_dir, &event_bus).unwrap();

    for pickle_event in pickle_events {
        runner.run_pickle(pickle_event, &event_bus);
    }
}

fn run_parallel_features(runner: Runner, event_bus: EventBus, config: &Config) {
    let pickle_events = parser::parse_pickle_events(&config.features_dir, &event_bus).unwrap();
    let mut pickle_events_per_feature = HashMap::new();

    for pickle_event in pickle_events {
        pickle_events_per_feature.entry(pickle_event.uri.clone())
            .or_insert(Vec::new())
            .push(pickle_event);
    }

    unimplemented!("run_parallel_features");
//    for (feature_uri, pickle_events) in pickle_events_per_feature {
//        pickle_events.into_par_iter()
//            .for_each(|pickle_event| runner.run_pickle(pickle_event, &event_bus));
//    }
}

fn run_parallel_scenarios(runner: Runner, event_bus: EventBus, config: &Config) {
    let pickle_events = parser::parse_pickle_events(&config.features_dir, &event_bus).unwrap();

    unimplemented!("run_parallel_scenarios");
//    pickle_events.into_par_iter()
//        .for_each(|pickle_event| runner.run_pickle(pickle_event, &event_bus));
}
