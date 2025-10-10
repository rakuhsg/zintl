use std::any::Any;

pub trait RenderObject {}

#[derive(Clone, Debug)]
pub struct RenderNode {
    pub object: usize,
    pub inner: Option<Box<RenderNode>>,
    pub children: Vec<RenderNode>,
}

impl RenderNode {
    pub fn new(index: usize) -> Self {
        RenderNode {
            object: index,
            inner: None,
            children: vec![],
        }
    }

    pub fn empty() -> Self {
        RenderNode {
            object: 0,
            inner: None,
            children: vec![],
        }
    }

    pub fn set_inner(&mut self, node: RenderNode) {
        self.inner = Some(Box::new(node));
    }

    pub fn push_child(&mut self, child: RenderNode) {
        self.children.push(child);
    }
}

#[derive(Debug)]
pub struct ROArena {
    cursor: usize,
    objects: Vec<Box<dyn Any>>,
}

impl ROArena {
    pub fn new() -> Self {
        let zero = Box::new(0); // Default render object

        ROArena {
            cursor: 0,
            objects: vec![zero],
        }
    }

    pub fn allocate(&mut self, object: Box<dyn Any>) -> usize {
        self.cursor += 1;
        self.objects.push(object);
        self.cursor
    }

    pub fn get_safe<T: 'static>(&self, index: usize) -> Option<&T> {
        if index == 0 {
            None
        } else if index <= self.cursor {
            self.objects[index].downcast_ref::<T>()
        } else {
            None
        }
    }
}
