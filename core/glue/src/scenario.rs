use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scenario {
    user_data: HashMap<TypeId, Box<Any>>,
}

impl Scenario {
    pub fn new() -> Scenario {
        Scenario {
            user_data: HashMap::new(),
        }
    }

    pub fn set_user_data<T: 'static>(&mut self, user_data: T) {
        self.user_data.insert(TypeId::of::<T>(), Box::new(user_data));
    }

    pub fn get_user_data<T: 'static>(&mut self) -> Option<&mut T> {
        self.user_data.get_mut(&TypeId::of::<T>())
            .map(|value| value.downcast_mut::<T>().unwrap())
    }
}
