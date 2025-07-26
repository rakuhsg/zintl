#[derive(Debug, Clone)]
pub enum Shape {
    Rectangle,
    Text { text: String, font_size: f32 },
}

#[derive(Debug, Clone, Default)]
pub enum RenderContent {
    #[default]
    Empty,
    Text(String),
    Image(String),
    Shape(Shape),
}

#[derive(Debug, Clone, Default)]
pub enum LayoutMode {
    Fixed,
    Flex,
    #[default]
    Block,
    Inline,
}

#[derive(Debug, Clone, Default)]
pub struct LayoutHint {
    pub mode: LayoutMode,
    pub metrics: Metrics,
}

#[derive(Debug, Clone, Default)]
pub enum Metrics {
    #[default]
    /// Automatically determine the size based on content or context
    Auto,
    /// Fixed width and height specified as (width, height)
    Fixed(f32, f32),
}

#[derive(Debug, Clone, Default)]
pub struct Position {
    x: f32,
    y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position { x, y }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

#[derive(Debug, Clone, Default)]
pub struct RenderObject {
    pub content: RenderContent,
    pub layout_hint: LayoutHint,
}

impl RenderObject {
    pub fn new(content: RenderContent, layout_hint: LayoutHint) -> Self {
        RenderObject {
            content,
            layout_hint,
        }
    }

    pub fn empty() -> Self {
        RenderObject::new(RenderContent::Empty, LayoutHint::default())
    }
}

#[derive(Debug, Clone, Default)]
pub struct RenderNode {
    pub object: RenderObject,
    pub inner: Option<Box<RenderNode>>,
    pub children: Vec<RenderNode>,
}

impl RenderNode {
    pub fn new(object: RenderObject) -> Self {
        RenderNode {
            object,
            inner: None,
            children: Vec::new(),
        }
    }

    pub fn set_inner(&mut self, node: RenderNode) {
        self.inner = Some(Box::new(node));
    }

    pub fn push_child(&mut self, child: RenderNode) {
        self.children.push(child);
    }
}

impl From<RenderObject> for RenderNode {
    fn from(object: RenderObject) -> Self {
        RenderNode::new(object)
    }
}
