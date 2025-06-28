use zintl_ui_render::RenderObject;
use zintl_ui_view::{Storage, View};

#[derive(Clone, Debug)]
pub struct App {
    storage: Storage,
    root: RenderObject,
}

impl App {
    pub fn new(mut view: impl View) -> Self {
        let mut storage = Storage::new();
        let root = view.render(&mut storage);
        App { storage, root }
    }
}
