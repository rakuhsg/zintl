use crate::runner::TestEvent;

pub enum TestRenderObject {
    Text(String),
    Empty,
}

pub struct TestBase {
    context: zintl::Context<TestEvent>,
}

impl TestBase {
    pub fn new() -> Self {
        TestBase {
            context: zintl::Context::new(),
        }
    }
}

impl zintl::View<TestEvent> for TestBase {
    fn get_context(&self) -> &zintl::Context<TestEvent> {
        &self.context
    }

    fn render(
        &mut self,
        arena: &mut zintl::ROArena,
        storage: &mut zintl::Storage,
        _: TestEvent,
    ) -> zintl::RenderNode {
        let idx = arena.allocate(Box::new(TestRenderObject::Empty));
        zintl::RenderNode::new(idx)
    }
}

pub struct TestLabel {
    context: zintl::Context<TestEvent>,
    text: String,
}

#[allow(dead_code)]
impl TestLabel {
    pub fn new(text: String) -> Self {
        TestLabel {
            text,
            context: zintl::Context::new(),
        }
    }
}

impl zintl::View<TestEvent> for TestLabel {
    fn get_context(&self) -> &zintl::Context<TestEvent> {
        &self.context
    }

    fn render(
        &mut self,
        arena: &mut zintl::ROArena,
        storage: &mut zintl::Storage,
        _: TestEvent,
    ) -> zintl::RenderNode {
        let idx = arena.allocate(Box::new(TestRenderObject::Text(self.text.clone())));
        zintl::RenderNode::new(idx)
    }
}

#[zintl::composable]
pub struct TestStack {
    context: zintl::Context<TestEvent>,
}

impl TestStack {
    pub fn new() -> Self {
        TestStack {
            context: zintl::Context::new(),
        }
    }
}

impl zintl::ComposableView<TestEvent> for TestStack {
    fn context(&self) -> &zintl::Context<TestEvent> {
        &self.context
    }

    fn compose(&mut self) -> impl zintl::View<TestEvent> {
        TestBase::new()
    }
}

pub struct TestButton<'a> {
    context: zintl::Context<TestEvent>,
    label: String,
    on_click_fn: Option<Box<dyn FnMut() -> () + 'a>>,
}

impl<'a> TestButton<'a> {
    pub fn new(label: String) -> Self {
        println!("TestButton::new()");
        TestButton {
            label,
            context: zintl::Context::new(),
            on_click_fn: None,
        }
    }

    pub fn on_click<T: FnMut() -> () + 'a>(mut self, f: T) -> Self {
        self.on_click_fn = Some(Box::new(f));
        self
    }
}

impl<'a> zintl::View<TestEvent> for TestButton<'a> {
    fn get_context(&self) -> &zintl::Context<TestEvent> {
        &self.context
    }

    fn render(
        &mut self,
        arena: &mut zintl::ROArena,
        storage: &mut zintl::Storage,
        event: TestEvent,
    ) -> zintl::RenderNode {
        match event {
            TestEvent::Click =>
                if let Some(f) = &mut self.on_click_fn {
                    println!("Click");
                    (f)();
                },
            _ => {}
        }

        let idx = arena.allocate(Box::new(TestRenderObject::Text(self.label.clone())));
        zintl::RenderNode::new(idx)
    }

    /*    fn handle(&mut self, event: TestEvent) {
        match event {
            TestEvent::Click =>
                if let Some(f) = &mut self.on_click_fn {
                    println!("Click");
                    (f)();
                },
            _ => {}
        }
    }*/
}
