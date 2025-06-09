use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::scaling::{
    DevicePixels, DevicePixelsF32, DeviceRect, DeviceSize, LogicalPixels, LogicalPoint,
    LogicalRect, ScaleFactor,
};
use crate::texture::Atlas;
use ab_glyph::{Font as _, ScaleFont};
use zintl_render_math::Vec2;

/// A rectangle and texture coordinates for a glyph.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GlyphRect {
    /// The width of the glyph in pixels.
    /// TODO: But actually this is physical pixels, not logical.
    pub width: LogicalPixels,
    /// The height of the glyph in pixels.
    /// TODO: But actually this is physical pixels, not logical.
    pub height: LogicalPixels,
    /// Mesh bounds on the glyph rectangle. NOT FOR LAYOUTING.
    /// TODO: But actually this is physical pixels, not logical.
    pub bounds: LogicalRect,
    /// The texture bounds of the glyph in the atlas.
    pub texture_bounds: DeviceRect,
}

/// A single glyph data with size and coordinates.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Glyph {
    /// ab_glyph glyph id.
    pub id: ab_glyph::GlyphId,
    pub rect: GlyphRect,
}

/// A font with a specific size.
#[derive(Clone, Debug)]
pub struct Font {
    pub ab_font: ab_glyph::FontArc,
    pub type_face: String,
    /// The atlas containing the rendered glyphs.
    pub atlas: Arc<Mutex<Atlas>>,
    /// The scale factor for the font.
    pub scale: DevicePixelsF32,
    pub height: DevicePixelsF32,
    /// https://docs.rs/ab_glyph/latest/ab_glyph/trait.Font.html#glyph-layout-concepts
    pub ascent: DevicePixelsF32,
    /// https://docs.rs/ab_glyph/latest/ab_glyph/trait.Font.html#glyph-layout-concepts
    pub descent: DevicePixelsF32,
    /// https://docs.rs/ab_glyph/latest/ab_glyph/trait.Font.html#glyph-layout-concepts
    pub line_gap: DevicePixelsF32,
    /// A cached list of glyphs in the atlas.
    pub glyphs: Arc<Mutex<HashMap<char, Glyph>>>,
}

impl Font {
    pub fn new(
        ab_font: ab_glyph::FontArc,
        type_face: String,
        scale: LogicalPixels,
        scale_factor: ScaleFactor,
    ) -> Self {
        let scale = scale * scale_factor.dpr;
        //let scale = scale.in_device_pixels(&scale_factor);
        let atlas = Atlas::new((scale as u32).max(1024), (scale as u32).max(32));
        let scale = scale as f32;
        // Init the font with PHYSICAL scale.
        let scaled = ab_font.as_scaled(scale);
        let height = scaled.height();
        let line_gap = scaled.line_gap();
        let ascent = scaled.ascent();
        let descent = scaled.descent();

        Font {
            ab_font,
            type_face,
            atlas: Mutex::new(atlas).into(),
            scale,
            height,
            ascent,
            descent,
            line_gap,
            glyphs: Mutex::new(HashMap::new()).into(),
        }
    }

    pub fn get_glyph(&self, c: char) -> Glyph {
        // TODO: Error handling
        if let Some(glyph) = self.glyphs.lock().unwrap().get(&c) {
            return *glyph;
        }

        let atlas = &mut self.atlas.lock().unwrap();
        // Init the font with PHYSICAL scale.
        let id = self.ab_font.glyph_id(c);

        if id.0 == 0 {
            return Glyph::default();
        }

        let scaled = self.ab_font.as_scaled(self.scale as f32);
        let h_advance = scaled.h_advance(id);

        let g = id.with_scale_and_position(
            self.scale,
            ab_glyph::Point {
                x: 0.0,
                y: scaled.ascent(),
            },
        );

        let rect = {
            if let Some(g) = self.ab_font.outline_glyph(g) {
                let px_bounds = g.px_bounds();

                // TODO: Properly scale the bounds
                let px_width = px_bounds.width() as DevicePixels;
                let px_height = px_bounds.height() as DevicePixels;
                // base: the absolute position in the atlas where the glyph will be drawn.
                let (texture_bounds, atlas_width, pixels) = atlas.create_image(px_width, px_height);
                g.draw(|x, y, c| {
                    if c == 0.0 {
                        return; // Skip transparent pixels
                    }
                    let start = (texture_bounds.min.y + y) * atlas_width * 4
                        + (texture_bounds.min.x + x) * 4;
                    let start = start as usize;
                    pixels[start] = ((1. - c) * 255.) as u8;
                    pixels[start + 1] = ((1. - c) * 255.) as u8;
                    pixels[start + 2] = ((1. - c) * 255.) as u8;
                    pixels[start + 3] = (c * 255.) as u8;
                });

                GlyphRect {
                    width: h_advance,
                    height: self.height,
                    bounds: LogicalRect {
                        min: LogicalPoint::new(px_bounds.min.x, px_bounds.min.y),
                        max: LogicalPoint::new(px_bounds.max.x, px_bounds.max.y),
                    },
                    texture_bounds,
                }
            } else {
                println!("Failed to get outline for glyph: {:?}", id);
                return Glyph::default();
            }
        };

        let glyph = Glyph { id, rect };

        println!("Glyph: {} {:?}", c, glyph);

        // Cache the glyph
        // TODO: Error handling
        self.glyphs.lock().unwrap().insert(c, glyph);

        glyph
    }

    pub fn kern(&self, left: &Glyph, right: &Glyph) -> LogicalPixels {
        let scaled = self.ab_font.as_scaled(self.scale);
        scaled.kern(left.id, right.id)
    }

    pub fn get_atlas_pixels(&self) -> Vec<u8> {
        self.atlas.lock().unwrap().pixels()
    }

    pub fn get_atlas_size(&self) -> DeviceSize {
        let atlas = self.atlas.lock().unwrap();
        DeviceSize::new(atlas.width, atlas.height)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Hash, Eq)]
pub struct FontProperties {
    pub name: String,
    /// f32 does not implement Hash, so we use a String instead.
    pub scale_string: String,
}

#[derive(Clone, Debug)]
pub struct Typecase {
    pub fonts: HashMap<String, ab_glyph::FontArc>,
    pub sized_fonts: HashMap<FontProperties, Font>,
    pub scale_factor: ScaleFactor,
}

impl Typecase {
    pub fn new(scale_factor: ScaleFactor) -> Self {
        Typecase {
            fonts: HashMap::new(),
            sized_fonts: HashMap::new(),
            scale_factor,
        }
    }

    pub fn load_font(&mut self, name: String, data: Vec<u8>) {
        let font = ab_glyph::FontVec::try_from_vec(data)
            .map(ab_glyph::FontArc::from)
            .unwrap();
        self.fonts.insert(name.clone(), font.clone());
    }

    pub fn get_font(&mut self, font: FontProperties) -> Option<&Font> {
        let f = self.sized_fonts.entry(font.clone()).or_insert({
            if let Some(ab_font) = self.fonts.get(&font.name) {
                let scale = font
                    .scale_string
                    .parse::<f32>()
                    .expect("Invalid scale string");
                let new_font =
                    Font::new(ab_font.clone(), font.name.clone(), scale, self.scale_factor);
                new_font
            } else {
                return None;
            }
        });
        Some(f)
    }
}

#[derive(Clone, Debug)]
pub struct PositionedGlyph {
    pub glyph: Glyph,
    pub rect: LogicalRect,
}

/// Composed glyphs, ready for rendering.
#[derive(Clone, Debug)]
pub struct Galley {
    pub glyphs: Vec<PositionedGlyph>,
    pub rect: LogicalRect,
}

#[derive(Clone, Debug)]
pub struct Typesetter {}

impl Typesetter {
    pub fn new() -> Self {
        Typesetter {}
    }

    /// Layouts a text
    pub fn compose(&self, text: &str, font: &Font, position: LogicalPoint) -> Galley {
        let position = LogicalPoint::new(position.x, position.y);
        let mut glyphs = Vec::new();
        let mut cursor = position.clone();
        let mut size: Vec2 = Vec2::default();
        let mut last_glyph_id = None;
        for c in text.chars() {
            let glyph = font.get_glyph(c);
            if let Some(last) = last_glyph_id {
                cursor.x += font.kern(&last, &glyph);
            }
            if size.y < glyph.rect.height {
                size.y = glyph.rect.height;
            }
            glyphs.push(PositionedGlyph {
                glyph,
                rect: LogicalRect::with_size(
                    cursor,
                    Vec2::new(glyph.rect.width, glyph.rect.height),
                ),
            });
            last_glyph_id = Some(glyph);
            cursor.x += glyph.rect.width;
            size.x += glyph.rect.width;
        }
        Galley {
            glyphs,
            rect: LogicalRect::with_size(position, size),
        }
    }
}
