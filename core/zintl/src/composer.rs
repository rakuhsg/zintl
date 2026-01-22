use crate::view::View;

pub struct Composer<R> {
    root: Box<dyn View<RenderNode = R>>,
}

impl<R> Composer<R> {
    pub fn new(root: Box<dyn View<RenderNode = R>>) -> Self {
        Composer { root }
    }

    pub fn render(&self) {}
}
