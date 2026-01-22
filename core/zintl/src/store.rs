use std::marker::PhantomData;

pub struct StoreId(usize);

pub struct Store<T> {
    id: StoreId,
    phantom: PhantomData<T>,
}
