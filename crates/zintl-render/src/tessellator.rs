use crate::mesh::Mesh;
use zintl::render::Shape;
pub struct Tessellator {}

impl Tessellator {
    pub fn new() -> Self {
        Tessellator {}
    }

    fn tessellate_text(&mut self, text: String, font_size: f32) -> Vec<Mesh> {
        vec![]
    }

    pub fn tessellate(&self, shape: &Shape) -> Vec<Mesh> {
        match shape {
            Shape::Rectangle => {
                unimplemented!()
            }
            //Shape::Text { text, font_size } => self.tessellate_text(text.clone(), *font_size),
            _ => {
                unimplemented!()
            }
        }
    }
}
