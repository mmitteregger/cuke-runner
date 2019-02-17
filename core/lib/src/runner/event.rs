use std::fmt::Debug;

use api::event::{Event, EventListener, SyncEventListener};

pub trait EventPublisher: Debug {
    fn send(&self, event: Event);
}

#[derive(Debug)]
pub(crate) struct EventBus<'a> {
    event_listeners: Vec<&'a EventListener>,
}

#[derive(Debug)]
pub(crate) struct SyncEventBus<'a> {
    event_listeners: Vec<&'a SyncEventListener>,
}

impl<'a> EventBus<'a> {
    pub fn new(event_listeners: Vec<&'a EventListener>) -> EventBus<'a> {
        EventBus {
            event_listeners,
        }
    }
}

impl<'a> SyncEventBus<'a> {
    pub fn new(event_listeners: Vec<&'a SyncEventListener>) -> SyncEventBus<'a> {
        SyncEventBus {
            event_listeners,
        }
    }
}

impl<'a> EventPublisher for EventBus<'a> {
    fn send(&self, event: Event) {
        for event_listener in &self.event_listeners {
            event_listener.on_event(&event);
        }
    }
}

impl<'a> EventPublisher for SyncEventBus<'a> {
    fn send(&self, event: Event) {
        for event_listener in &self.event_listeners {
            event_listener.on_event(&event);
        }
    }
}
