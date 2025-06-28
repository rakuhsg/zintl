use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

/// Persistent key-value storage for [`View`].
#[derive(Clone, Debug)]
pub struct Storage {
    data: Arc<HashMap<String, Arc<dyn Any>>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            data: HashMap::new().into(),
        }
    }
}
