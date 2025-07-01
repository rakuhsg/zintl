use crate::render::RenderNode;
use crate::view::{Storage, View};

// TODO
#[allow(unused)]
#[derive(Clone, Debug)]
pub struct App {
    storage: Storage,
    root: RenderNode,
}

impl App {
    pub fn new(mut view: impl View) -> Self {
        let mut storage = Storage::new();
        let root = view.render(&mut storage);
        println!("{:?}", root);
        App { storage, root }
    }
}
