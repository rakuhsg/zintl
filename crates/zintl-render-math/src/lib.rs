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

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn add(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }

    pub fn subtract(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }

    pub fn multiply(&self, scalar: f32) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }

    pub fn min(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x.min(other.x), self.y.min(other.y))
    }

    pub fn max(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x.max(other.x), self.y.max(other.y))
    }
}

impl From<(usize, usize)> for Vec2 {
    fn from(tuple: (usize, usize)) -> Self {
        Vec2::new(tuple.0 as f32, tuple.1 as f32)
    }
}

/// Matrix 3x3
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Mat3 {
    pub m: [[f32; 3]; 3],
}

/// Matrix 4x4
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Mat4 {
    pub m: [[f32; 4]; 4],
}

impl From<Mat4> for cgmath::Matrix4<f32> {
    fn from(mat: Mat4) -> Self {
        cgmath::Matrix4::from(mat.m)
    }
}

impl From<cgmath::Matrix4<f32>> for Mat4 {
    fn from(mat: cgmath::Matrix4<f32>) -> Self {
        Mat4 { m: mat.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        let result = v1.add(&v2);
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
    }

    #[test]
    fn test_subtract() {
        let v1 = Vec2::new(5.0, 6.0);
        let v2 = Vec2::new(3.0, 4.0);
        let result = v1.subtract(&v2);
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 2.0);
    }

    #[test]
    fn test_multiply() {
        let v = Vec2::new(1.0, 2.0);
        let result = v.multiply(3.0);
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 6.0);
    }
}
