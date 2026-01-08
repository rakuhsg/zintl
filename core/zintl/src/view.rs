pub struct Context {}

pub trait View {
    fn init(&mut self, cx: &mut Context);
    fn render(&self, cx: &mut Context);
}
