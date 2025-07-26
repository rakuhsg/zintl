#[derive(Default)]
pub struct TestLabel {
    context: zintl::Context,
    text: String,
}

#[allow(dead_code)]
impl TestLabel {
    pub fn new(text: String) -> Self {
        TestLabel {
            text,
            ..Default::default()
        }
    }
}

impl zintl::Composable for TestLabel {
    fn context(&self) -> &zintl::Context {
        &self.context
    }
    fn compose(&mut self) -> impl zintl::View {
        zintl_widget::Label::new(self.text.clone())
    }
}
