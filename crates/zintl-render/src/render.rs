use zintl_render_math::{Point, Vec2};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    /// The size on the screen
    pub size: Vec2,
    /// The top-left coordinate of the texture
    pub tex_coords: Point,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Glyph {
    pub glyph_id: ab_glyph::GlyphId,
    pub width: f32,
    pub rect: Rect,
}
