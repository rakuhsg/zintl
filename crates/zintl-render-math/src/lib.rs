mod mat;
mod vec;

pub use self::{mat::*, vec::*};

/// A type alias for a value in pixels.
#[deprecated(since = "0.1.0", note = "Use `LogicalPixel` instead")]
pub type Pixel = f32;

/// Do not use this to logically scale pixels.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[deprecated(since = "0.1.0", note = "Use `scaling::LogicalPoint` instead")]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}
