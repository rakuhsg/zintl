#[derive(Copy, Debug, Clone)]
pub struct Rect {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

pub struct RenderNode {
    pub pos_x: i32,
    pub pos_y: i32,
    pub text: String,
}

impl RenderNode {
    pub fn new(text: String, pos_x: i32, pos_y: i32) -> Self {
        RenderNode { text, pos_x, pos_y }
    }
}
