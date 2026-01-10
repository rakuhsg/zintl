use std::marker::PhantomData;

pub struct Store<T> {
    phantom: PhantomData<T>,
}
