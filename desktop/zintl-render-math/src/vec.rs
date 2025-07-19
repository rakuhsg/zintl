#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    pub const X_AXIS: Vec2 = Vec2 { x: 1.0, y: 0.0 };
    pub const Y_AXIS: Vec2 = Vec2 { x: 0.0, y: 1.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn checked_div(self, other: Vec2) -> Option<Vec2> {
        if other.x == 0.0 || other.y == 0.0 {
            None
        } else {
            Some(Vec2::new(self.x / other.x, self.y / other.y))
        }
    }

    pub fn checked_div_f32(self, scalar: f32) -> Option<Vec2> {
        if scalar == 0.0 {
            None
        } else {
            Some(Vec2::new(self.x / scalar, self.y / scalar))
        }
    }

    pub fn min(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x.min(other.x), self.y.min(other.y))
    }

    pub fn max(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x.max(other.x), self.y.max(other.y))
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, vec: Vec2) -> Vec2 {
        Vec2::new(self * vec.x, self * vec.y)
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl From<(usize, usize)> for Vec2 {
    fn from(tuple: (usize, usize)) -> Self {
        Vec2::new(tuple.0 as f32, tuple.1 as f32)
    }
}

// TODO: Write tests for Vec2
