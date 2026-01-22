pub trait Component {
    type RenderNode;

    fn render(&self) -> Self::RenderNode;
}
