use std::cell::RefCell;

#[derive(Debug, Clone)]
pub enum Shape {
    Rectangle,
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
    Auto,
    Fixed(f32, f32),
    Absolute(f32, f32),
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
    content: RenderContent,
    position: Position,
    metrics: Metrics,
    children: RefCell<Vec<RenderObject>>,
}

impl RenderObject {
    pub fn new(content: RenderContent, position: Position, metrics: Metrics) -> Self {
        RenderObject {
            content,
            position,
            metrics,
            children: Vec::new().into(),
        }
    }

    pub fn add_child(&self, child: RenderObject) {
        self.children.borrow_mut().push(child);
    }
}
