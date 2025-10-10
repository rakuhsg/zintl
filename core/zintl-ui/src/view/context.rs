use std::cell::RefCell;

use crate::{
    event::Event,
    render::{ROArena, RenderNode},
    view::Storage,
};

pub type Generator<E: Event> = Box<dyn FnMut(&mut ROArena, &mut Storage, E) -> RenderNode>;

/// The context consists of a set of style properties and layouts to render
/// views.
#[derive(Default)]
pub struct Context<E: Event> {
    children: RefCell<Vec<Generator<E>>>,
}

impl<E: Event> std::fmt::Debug for Context<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Context {{ ... }}")
    }
}

impl<E: Event> Context<E> {
    pub fn new() -> Self {
        Context {
            children: Vec::new().into(),
        }
    }

    pub fn set_style_property(&self) {}

    pub fn render_children(
        &self,
        arena: &mut ROArena,
        storage: &mut Storage,
        event: E,
    ) -> RenderNode {
        let mut node = RenderNode::empty();
        let mut children = self.children.borrow_mut();
        for child in &mut *children {
            node.push_child(child(arena, storage, event.clone()));
        }
        node
    }

    pub fn set_children(&self, children: Vec<Generator<E>>) {
        self.children.replace(children);
    }
}
