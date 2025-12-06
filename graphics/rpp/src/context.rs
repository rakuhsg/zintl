use crate::shader::*;

pub trait Context {
    fn create_solid_color_shader(&mut self) -> ShaderId;
    fn set_shader(&mut self, id: ShaderId);
    fn fill_rect();
}
