pub struct Context {}

pub trait View {
    type RenderNode;

    fn init(&mut self, cx: &mut Context);
    fn render(&self, cx: &mut Context) -> Self::RenderNode;
}
