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
    pub position: Position,
    pub metrics: Metrics,
}

impl RenderObject {
    pub fn new(content: RenderContent, position: Position, metrics: Metrics) -> Self {
        RenderObject {
            content,
            position,
            metrics,
        }
    }

    pub fn empty() -> Self {
        RenderObject::new(RenderContent::Empty, Position::new(0., 0.), Metrics::Auto)
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
