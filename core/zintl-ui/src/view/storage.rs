use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, Mutex},
};

/// Persistent key-value storage for [`View`].
#[derive(Clone, Debug)]
pub struct Storage {
    data: Arc<Mutex<HashMap<String, Arc<dyn Any>>>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            data: Mutex::new(HashMap::new()).into(),
        }
    }

    pub fn insert<T: 'static>(&mut self, key: String, data: T) {
        self.data
            .lock()
            .unwrap()
            .insert(key, Arc::new(data))
            .unwrap();
    }
}
