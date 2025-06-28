use zintl_ui::{Context, Metrics, Position, RenderContent, RenderObject, Storage, View};

pub struct Label {
    context: Context,
    text: String,
}

impl Label {
    pub fn new(text: String) -> Self {
        Label {
            context: Context::default(),
            text,
        }
    }
}

impl View for Label {
    fn get_context(&self) -> &Context {
        &self.context
    }

    fn render(&mut self, _: &mut Storage) -> RenderObject {
        RenderObject::new(
            RenderContent::Text(self.text.clone()),
            Position::new(0., 0.),
            Metrics::Auto,
        )
    }
}
