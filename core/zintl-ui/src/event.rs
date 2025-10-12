pub trait Event: Clone {
    fn initial() -> Self;
}
