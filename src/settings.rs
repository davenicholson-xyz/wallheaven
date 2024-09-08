use std::{any::Any, collections::HashMap};

#[derive(Debug)]
pub struct Settings {
    pub params: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            params: HashMap::new(),
        }
    }
    pub fn set<T: 'static + Send + Sync>(&mut self, key: &str, value: T) {
        self.params.insert(key.to_string(), Box::new(value));
    }
    pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
        self.params.get(key)?.downcast_ref::<T>()
    }
}
