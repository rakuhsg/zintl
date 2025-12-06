use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, RwLock},
};

/// Persistent key-value storage for [`View`].
#[derive(Clone, Debug)]
pub struct Storage {
    store: Arc<RwLock<HashMap<String, Box<dyn Any>>>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            store: RwLock::new(HashMap::new()).into(),
        }
    }

    pub fn insert<T: Any>(&mut self, key: String, value: Box<T>) {
        //FIXME: this unwrap() is dangerous.
        self.store.write().unwrap().insert(key, value);
    }

    /// Motifies value with closure and return its value. Returns None when
    /// entry is empty.
    #[inline]
    pub fn modify<T: Any, R, F>(&mut self, key: String, f: F) -> Option<R>
    where
        F: FnOnce(Option<&mut T>) -> Option<R>,
    {
        let mut res: Option<R> = None;
        self.store.write().unwrap().entry(key).and_modify(|e| {
            let val = e.downcast_mut::<T>();
            res = f(val);
        });
        res
    }
}
