use crate::context::{Context, Generator};
use crate::storage::Storage;
use crate::view::View;

use zintl_ui_render::{RenderNode, RenderObject};

/// A view that implements the [`Composable`] trait.
pub trait Composable: Sized {
    fn context(&self) -> &Context;
    fn compose(&mut self) -> impl View;

    fn children(self, children: Vec<Generator>) -> Self {
        self.get_context().set_children(children);
        self
    }
}

impl<T: Composable> View for T {
    fn get_context(&self) -> &Context {
        self.context()
    }

    fn render(&mut self, storage: &mut Storage) -> RenderNode {
        let mut node = RenderNode::new(RenderObject::empty());
        node.set_inner(self.compose().render(storage));
        println!("{:?}", node);
        let child = self.get_context().render_children(storage);
        node.push_child(child);
        node
    }
}
