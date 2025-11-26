use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, RwLock},
};

/// Persistent key-value storage for [`View`].
#[derive(Clone, Debug)]
pub struct Storage {
    store: Arc<RwLock<HashMap<String, Arc<dyn Any>>>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            store: RwLock::new(HashMap::new()).into(),
        }
    }

    pub fn insert<T: Any>(&mut self, key: String, data: T) {
        //FIXME: this unwrap() is dangerous.
        self.store.write().unwrap().insert(key, Arc::new(data));
    }
}
