use zintl_ui::{
    Composable, Context, Metrics, Position, RenderContent, RenderNode, RenderObject, Storage, View,
};

pub struct Base {
    context: Context,
}

impl Base {
    pub fn new() -> Self {
        Base {
            context: Context::default(),
        }
    }
}

impl View for Base {
    fn get_context(&self) -> &Context {
        &self.context
    }
}

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

    fn render(&mut self, _: &mut Storage) -> RenderNode {
        RenderObject::new(
            RenderContent::Text(self.text.clone()),
            Position::new(0., 0.),
            Metrics::Auto,
        )
        .into()
    }
}

pub struct Stack {
    context: Context,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            context: Context::default(),
        }
    }
}

impl Composable for Stack {
    fn context(&self) -> &Context {
        &self.context
    }

    fn compose(&mut self) -> impl View {
        Base::new()
    }
}
