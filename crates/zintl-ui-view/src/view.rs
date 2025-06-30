use crate::context::Context;
use crate::storage::Storage;

use zintl_ui_render::{RenderNode, RenderObject};

// A renderable component that has context.
pub trait View: Sized {
    fn get_context(&self) -> &Context;

    fn render(&mut self, _: &mut Storage) -> RenderNode {
        RenderNode::new(RenderObject::empty())
    }

    // TODO
    #[allow(unused)]
    fn padding(self, top: f32, bottom: f32, left: f32, right: f32) -> Self {
        self.get_context().set_style_property();
        self
    }
}
