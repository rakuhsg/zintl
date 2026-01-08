pub trait Hook {
    type Message;
    fn set_id(&mut self, id: usize);
    fn get_id(&self) -> usize;
}
