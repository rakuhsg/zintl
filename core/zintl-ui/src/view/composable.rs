use crate::{
    event::Event,
    render::{ROArena, RenderNode},
    view::{Context, Generator, Storage, View},
};

/// A marker trait for views that have children.
pub trait Composable {}

/// A view that implements the [`Composable`] trait.
pub trait ComposableView<E: Event>: Sized + Composable + View<E> {
    fn context(&self) -> &Context<E>;
    fn compose(&mut self) -> impl View<E>;

    fn children<F: FnOnce() -> Vec<Generator<E>>>(self, f: F) -> Self {
        self.get_context().set_children(f());
        self
    }
}

impl<T, E> View<E> for T
where
    E: Event,
    T: ComposableView<E> + Composable,
{
    fn get_context(&self) -> &Context<E> {
        self.context()
    }

    fn render(&mut self, arena: &mut ROArena, storage: &mut Storage, event: E) -> RenderNode {
        let mut node = RenderNode::empty();
        node.set_inner(self.compose().render(arena, storage, event.clone()));
        let child = self.get_context().render_children(arena, storage, event);
        node.push_child(child);
        node
    }
}
