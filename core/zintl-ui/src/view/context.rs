use std::cell::RefCell;

use crate::render::{RenderNode, RenderObject};
use crate::view::Storage;

pub type Generator = Box<dyn FnMut(&mut Storage) -> RenderNode>;

/// The context consists of a set of style properties and layouts to render views.
#[derive(Default)]
pub struct Context {
    children: RefCell<Vec<Generator>>,
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Context {{ ... }}")
    }
}

impl Context {
    pub fn new() -> Self {
        Context {
            children: Vec::new().into(),
        }
    }
    pub fn set_style_property(&self) {}
    pub fn render_children(&self, storage: &mut Storage) -> RenderNode {
        let mut node = RenderNode::new(RenderObject::empty());
        let mut children = self.children.borrow_mut();
        for child in &mut *children {
            node.push_child(child(storage));
        }
        node
    }
    pub fn set_children(&self, children: Vec<Generator>) {
        self.children.replace(children);
    }
}
