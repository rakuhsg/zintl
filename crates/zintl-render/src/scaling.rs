pub struct Viewport {
    /// A device width. (non-logical, in physical pixels)
    pub device_width: f32,
    /// A device height. (non-logical, in physical pixels)
    pub device_height: f32,
    pub scale_factor: ScaleFactor,
}

pub struct ScaleFactor {
    pub device_pixel_ratio: f32,
}
