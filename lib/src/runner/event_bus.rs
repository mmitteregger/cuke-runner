use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;

use api::event::{Event, EventListener};

//#[derive(Debug, Default)]
//pub struct EventBus {
//    event_handlers: RefCell<Vec<Box<EventListener>>>,
//}
//
//impl EventBus {
//    pub fn new() -> EventBus {
//        EventBus::default()
//    }
//
//    pub fn send(&self, event: Event) {
//        for event_handler in self.event_handlers.borrow_mut().iter_mut() {
//            event_handler.on_event(&event);
//        }
//    }
//
//    pub fn register_listener<L: EventListener + 'static>(&mut self, listener: L) -> &mut L {
//        let boxed_listener = Box::new(listener);
//        let boxed_listener_ptr = Box::into_raw(boxed_listener);
//        self.event_handlers.borrow_mut().push(unsafe { Box::from_raw(boxed_listener_ptr) });
//        unsafe { &mut *boxed_listener_ptr }
//    }
//}

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
