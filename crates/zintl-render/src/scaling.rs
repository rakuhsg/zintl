use zintl_render_math::Vec2;

pub type DevicePixels = u32;

pub trait DeviceScale {
    fn in_logical_pixels(self, scale_factor: &ScaleFactor) -> LogicalPixels;
}

impl DeviceScale for DevicePixels {
    fn in_logical_pixels(self, scale_factor: &ScaleFactor) -> LogicalPixels {
        (self as f32 / scale_factor.device_pixel_ratio).round() as LogicalPixels
    }
}

pub type DevicePixelsF32 = f32;

impl DeviceScale for DevicePixelsF32 {
    fn in_logical_pixels(self, scale_factor: &ScaleFactor) -> LogicalPixels {
        (self / scale_factor.device_pixel_ratio).round() as LogicalPixels
    }
}

pub type LogicalPixels = f32;

pub trait LogicalScale {
    fn in_device_pixels(self, scale_factor: &ScaleFactor) -> DevicePixels;
}

impl LogicalScale for LogicalPixels {
    fn in_device_pixels(self, scale_factor: &ScaleFactor) -> DevicePixels {
        (self * scale_factor.device_pixel_ratio).round() as DevicePixels
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    /// A device pixels width. (non-logical, in physical pixels)
    pub device_width: DevicePixels,
    /// A device pixels height. (non-logical, in physical pixels)
    pub device_height: DevicePixels,
    pub scale_factor: ScaleFactor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScaleFactor {
    pub device_pixel_ratio: f32,
}

/// A Normalized Texture Point.
/// This is a point in the range [0.0, 1.0] for both x and y coordinates.
/// Do not use this to logically scale pixels.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TexturePoint {
    x: f32,
    y: f32,
}

impl TexturePoint {
    pub fn new(x: f32, y: f32) -> Self {
        assert!(x >= 0.0 && x <= 1.0, "TexturePoint x must be in [0.0, 1.0]");
        assert!(y >= 0.0 && y <= 1.0, "TexturePoint y must be in [0.0, 1.0]");

        TexturePoint { x, y }
    }

    pub fn from_device_point(point: DevicePoint, texture_size: DeviceSize) -> Self {
        let x = point.x as f32 / texture_size.width as f32;
        let y = point.y as f32 / texture_size.height as f32;
        TexturePoint::new(x, y)
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

/// A Normalized Texture Rect.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TextureBounds {
    pub min: TexturePoint,
    pub max: TexturePoint,
}

impl TextureBounds {
    pub fn new(min: TexturePoint, max: TexturePoint) -> Self {
        TextureBounds { min, max }
    }

    pub fn from_device_rect(rect: DeviceRect, texture_size: DeviceSize) -> Self {
        TextureBounds {
            min: TexturePoint::from_device_point(rect.min, texture_size),
            max: TexturePoint::from_device_point(rect.max, texture_size),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DevicePointF32 {
    pub x: f32,
    pub y: f32,
}

impl From<DevicePoint> for DevicePointF32 {
    fn from(point: DevicePoint) -> Self {
        DevicePointF32 {
            x: point.x as f32,
            y: point.y as f32,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DevicePoint {
    pub x: DevicePixels,
    pub y: DevicePixels,
}

impl DevicePoint {
    pub fn new(x: DevicePixels, y: DevicePixels) -> Self {
        DevicePoint { x, y }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DeviceRect {
    pub min: DevicePoint,
    pub max: DevicePoint,
}

impl DeviceRect {
    pub fn new(min: DevicePoint, max: DevicePoint) -> Self {
        DeviceRect { min, max }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DeviceSize {
    pub width: DevicePixels,
    pub height: DevicePixels,
}

impl DeviceSize {
    pub fn new(width: DevicePixels, height: DevicePixels) -> Self {
        DeviceSize { width, height }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LogicalPoint {
    pub x: LogicalPixels,
    pub y: LogicalPixels,
}

impl LogicalPoint {
    pub fn new(x: LogicalPixels, y: LogicalPixels) -> Self {
        LogicalPoint { x, y }
    }

    pub fn scale(&self, device_pixel_ratio: f32) -> DevicePoint {
        DevicePoint {
            x: (self.x * device_pixel_ratio).round() as DevicePixels,
            y: (self.y * device_pixel_ratio) as DevicePixels,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LogicalRect {
    pub min: LogicalPoint,
    pub max: LogicalPoint,
}

impl LogicalRect {
    pub fn new(min: LogicalPoint, max: LogicalPoint) -> Self {
        Self { min, max }
    }

    pub fn scale(&self, device_pixel_ratio: f32) -> DeviceRect {
        DeviceRect {
            min: self.min.scale(device_pixel_ratio),
            max: self.max.scale(device_pixel_ratio),
        }
    }

    pub fn with_size(min: LogicalPoint, size: Vec2) -> Self {
        let max = LogicalPoint::new(min.x + size.x, min.y + size.y);
        Self { min, max }
    }

    pub fn zero() -> Self {
        Self {
            min: LogicalPoint::new(0.0, 0.0),
            max: LogicalPoint::new(0.0, 0.0),
        }
    }

    pub fn normalize(&self, size: Vec2) -> Self {
        let min = LogicalPoint::new(self.min.x / size.x, self.min.y / size.y);
        let max = LogicalPoint::new(self.max.x / size.x, self.max.y / size.y);
        Self { min, max }
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }
}
