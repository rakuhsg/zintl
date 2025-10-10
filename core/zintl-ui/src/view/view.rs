use crate::{
    event::Event,
    render::{ROArena, RenderNode},
    view::{Context, Storage},
};

// A renderable component that has context.
pub trait View<E>
where
    Self: Sized,
    E: Event,
{
    fn get_context(&self) -> &Context<E>;

    fn render(&mut self, arena: &mut ROArena, storage: &mut Storage, event: E) -> RenderNode;

    fn handle(&mut self, e: E) {}

    // TODO
    #[allow(unused)]
    fn padding(self, top: f32, bottom: f32, left: f32, right: f32) -> Self {
        self.get_context().set_style_property();
        self
    }
}
