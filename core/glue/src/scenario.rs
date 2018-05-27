use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scenario {
    user_types: HashMap<TypeId, Box<Any>>,
}

impl Scenario {
    pub fn new() -> Scenario {
        Scenario {
            user_types: HashMap::new(),
        }
    }

    pub fn get_by_type<T: Any>(&mut self) -> Option<&mut T> {
        self.user_types.get_mut(&TypeId::of::<T>())
            .map(|value| value.downcast_mut::<T>().unwrap())
    }
}
