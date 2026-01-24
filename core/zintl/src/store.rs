use std::marker::PhantomData;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StoreId(usize);

impl StoreId {
    pub const UNINITIALIZED: Self = StoreId(0);
}

pub struct Store<T> {
    id: StoreId,
    phantom: PhantomData<T>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Store {
            id: StoreId::UNINITIALIZED,
            phantom: PhantomData,
        }
    }

    pub(crate) fn init(&mut self, id: StoreId) {
        self.id = id;
    }
}
