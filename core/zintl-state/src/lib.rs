use zintl_ui::RenderNode;
use zintl_ui::{Context, Storage, View};

pub struct State<T> {
    value: T,
}

#[derive(Debug)]
pub struct StatefulView<T> {
    context: Context,
    key: String,
    state: T,
}

impl<T> StatefulView<T> {
    pub fn new(key: String, initial: T) -> Self {
        StatefulView {
            context: Context::default(),
            key,
            state: initial,
        }
    }

    //TODO: children
}

impl<T> View for StatefulView<T> {
    fn get_context(&self) -> &Context {
        &self.context
    }

    fn render(&mut self, _storage: &mut Storage) -> RenderNode {
        unimplemented!()
    }
}
