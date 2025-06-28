use std::cell::RefCell;

use crate::storage::Storage;
use zintl_ui_render::{Metrics, Position, RenderContent, RenderObject};

pub type Generator = Box<dyn Fn(&mut Storage) -> RenderObject>;

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
    pub fn render(&self, storage: &mut Storage) -> RenderObject {
        let children = self.children.borrow();
        let mut objs = Vec::new();
        for child in &*children {
            objs.push(child(storage));
        }
        // TODO: Rendering itself
        let robj = RenderObject::new(RenderContent::Empty, Position::new(0.0, 0.0), Metrics::Auto);
        robj.set_children(objs);
        robj
    }
    pub fn set_children(&self, children: Vec<Generator>) {
        self.children.replace(children);
    }
}
