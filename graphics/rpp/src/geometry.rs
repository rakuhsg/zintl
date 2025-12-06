pub type Float = f32;

pub trait ZeroValue {
    fn zero() -> Self;
}

impl ZeroValue for Float {
    #[inline(always)]
    fn zero() -> Self {
        0.0_f32
    }
}

pub struct Point {
    pub p: [Float; 2],
}

impl Point {
    #[inline(always)]
    pub fn new(x: Float, y: Float) -> Self {
        Point { p: [x, y] }
    }

    #[inline(always)]
    pub fn x(&self) -> Float {
        self.p[0]
    }

    #[inline(always)]
    pub fn y(&self) -> Float {
        self.p[1]
    }

    #[inline(always)]
    pub fn distance_from(&self, other: &Point) -> Float {
        ((other.x() - self.x()).powi(2) + (other.y() - self.y()).powi(2)).sqrt()
    }
}

impl ZeroValue for Point {
    #[inline(always)]
    fn zero() -> Self {
        Point {
            p: [Float::zero(), Float::zero()],
        }
    }
}

impl From<[Float; 2]> for Point {
    #[inline(always)]
    fn from(p: [Float; 2]) -> Self {
        Point { p }
    }
}

impl From<Point> for [Float; 2] {
    #[inline(always)]
    fn from(value: Point) -> Self {
        value.p
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Rect {
    pub r: [[Float; 2]; 2],
}

impl Rect {
    #[inline(always)]
    pub fn new(min: Point, max: Point) -> Self {
        Rect { r: [min.p, max.p] }
    }

    #[inline(always)]
    pub fn from_float(minx: f32, miny: f32, maxx: f32, maxy: f32) -> Self {
        Rect {
            r: [[minx, miny], [maxx, maxy]],
        }
    }

    #[inline(always)]
    pub fn min(&self) -> Point {
        self.r[0].into()
    }

    #[inline(always)]
    pub fn max(&self) -> Point {
        self.r[1].into()
    }
}

impl ZeroValue for Rect {
    #[inline(always)]
    fn zero() -> Self {
        Rect {
            r: [Point::zero().p, Point::zero().p],
        }
    }
}

impl From<[[Float; 2]; 2]> for Rect {
    #[inline(always)]
    fn from(r: [[Float; 2]; 2]) -> Self {
        Rect { r }
    }
}

impl From<Rect> for [[Float; 2]; 2] {
    #[inline(always)]
    fn from(value: Rect) -> Self {
        value.r
    }
}

impl From<[Float; 4]> for Rect {
    #[inline(always)]
    fn from(value: [Float; 4]) -> Self {
        Rect {
            r: [[value[0], value[1]], [value[2], value[3]]],
        }
    }
}

impl From<Rect> for [Float; 4] {
    #[inline(always)]
    fn from(value: Rect) -> Self {
        let r = value.r;
        [r[0][0], r[0][1], r[1][0], r[1][1]]
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Size {
    pub s: [Float; 2],
}

impl Size {
    #[inline(always)]
    pub fn new(width: Float, height: Float) -> Self {
        Size { s: [width, height] }
    }

    #[inline(always)]
    pub fn width(&self) -> Float {
        self.s[0]
    }

    #[inline(always)]
    pub fn height(&self) -> Float {
        self.s[1]
    }
}

/// Matrix 3x3
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Mat3 {
    pub m: [[Float; 3]; 3],
}

/// Matrix 4x4
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Mat4 {
    pub m: [[Float; 4]; 4],
}

impl From<[[Float; 4]; 4]> for Mat4 {
    fn from(m: [[Float; 4]; 4]) -> Self {
        Mat4 { m }
    }
}

impl From<Mat4> for [[Float; 4]; 4] {
    fn from(value: Mat4) -> Self {
        value.m
    }
}

pub struct Viewport {
    pub width: Float,
    pub height: Float,
    scale_factor: Float,
}

impl Viewport {
    pub fn to_ortho(&self) -> Mat4 {
        let width = self.width * self.scale_factor;
        let height = self.height * self.scale_factor;
        let top = Float::zero();
        let bottom = height;
        let left = Float::zero();
        let right = width;
        let far = 1.0;
        let near = -1.0;

        [
            [2.0 / (left - right), 0.0, 0.0, 0.0],
            [0.0, 2.0 / (top - bottom), 0.0, 0.0],
            [0.0, 0.0, -2.0 / (far - near), 0.0],
            [
                -(left + right) / (left - right),
                -(top + bottom) / (top - bottom),
                -(far + near) / (far - near),
                1.0,
            ],
        ]
        .into()
    }
}
