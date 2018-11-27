use std::cell::RefCell;

use api::event::{Event, EventListener};

#[derive(Debug, Default)]
pub struct EventBus<'a> {
    event_handlers: RefCell<Vec<&'a mut EventListener>>,
}

impl<'a> EventBus<'a> {
    pub fn new() -> EventBus<'a> {
        EventBus::default()
    }

    pub fn send(&self, event: Event) {
        for event_handler in self.event_handlers.borrow_mut().iter_mut() {
            event_handler.on_event(&event);
        }
    }

    pub fn register_listener(&mut self, listener: &'a mut EventListener) {
        self.event_handlers.borrow_mut().push(listener);
    }
}
