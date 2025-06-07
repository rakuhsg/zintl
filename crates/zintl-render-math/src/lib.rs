mod mat;
mod vec;

pub use self::{mat::*, vec::*};

/// A type alias for a value in pixels.
pub type Pixel = f32;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PointUSize {
    pub x: usize,
    pub y: usize,
}

impl PointUSize {
    pub fn new(x: usize, y: usize) -> Self {
        PointUSize { x, y }
    }
}

impl From<PointUSize> for Point {
    fn from(point: PointUSize) -> Self {
        Point::new(point.x as f32, point.y as f32)
    }
}
