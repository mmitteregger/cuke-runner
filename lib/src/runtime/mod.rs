pub use self::hook_definition::*;
pub use self::step_definition::*;
pub use self::glue::*;
pub use self::runtime_options::*;
pub use self::tag_predicate::*;
pub use self::test_case::*;
pub use self::scenario::*;
pub use self::step_definition_match::*;
pub use self::definition_argument::*;
pub use self::exit_status::*;

use runner::{Runner, EventBus};
use self::exit_status::ExitStatusListener;
use api::event::EventListener;

pub mod step_expression;
mod hook_definition;
mod step_definition;
mod glue;
mod runtime_options;
mod tag_predicate;
mod test_case;
mod scenario;
mod step_definition_match;
mod definition_argument;
mod exit_status;


//pub struct Runtime<'a> {
//    event_bus: EventBus<'a>,
//    runner: Runner,
//    exit_status_listener: ExitStatusListener,
//}
//
//impl<'a> Runtime<'a> {
//    pub fn new(glue: Glue, runtime_options: RuntimeOptions) -> Runtime<'a> {
//        let mut event_bus = EventBus::new();
//
//        let mut exit_status_listener = ExitStatusListener::new();
//        let runner = Runner::new(glue, runtime_options);
//
//        let runtime = Runtime {
//            event_bus,
//            runner,
//            exit_status_listener,
//        };
//
//        runtime.
//
//        runtime
//    }
//}

pub fn run(glue: Glue, runtime_options: RuntimeOptions) -> i32 {
    let strict = runtime_options.strict;
    let mut exit_status_listener = ExitStatusListener::new();

    {
        let mut event_bus = EventBus::new();
        event_bus.register_listener(&mut exit_status_listener);

        let runner = Runner::new(glue, runtime_options);
    }

    let exit_status = exit_status_listener.get_exit_status(strict);
    exit_status
}
