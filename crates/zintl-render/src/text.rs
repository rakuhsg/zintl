use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use zintl_render_math::{
    Alignment, InLogicalScale, InPhysicalScale, LogicalPixels, LogicalPixelsRect, PhysicalPixels,
    PhysicalPixelsF, PhysicalPixelsFPoint, PhysicalPixelsFRect, PhysicalPixelsFSize,
    PhysicalPixelsRect, PhysicalPixelsSize, ScaleFactor,
};

use crate::texture::Atlas;
use ab_glyph::{Font as _, ScaleFont};

/// A rectangle and texture coordinates for a glyph.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GlyphRect {
    /// The width of the glyph in pixels.
    pub width: PhysicalPixelsF,
    /// The height of the glyph in pixels.
    pub height: PhysicalPixelsF,
    /// Mesh bounds on the glyph rectangle. NOT FOR LAYOUTING.
    pub bounds: PhysicalPixelsFRect,
    /// The texture bounds of the glyph in the atlas.
    pub texture_bounds: PhysicalPixelsRect,
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
    pub scale: PhysicalPixelsF,
    pub height: PhysicalPixelsF,
    /// https://docs.rs/ab_glyph/latest/ab_glyph/trait.Font.html#glyph-layout-concepts
    pub ascent: PhysicalPixelsF,
    /// https://docs.rs/ab_glyph/latest/ab_glyph/trait.Font.html#glyph-layout-concepts
    pub descent: PhysicalPixelsF,
    /// https://docs.rs/ab_glyph/latest/ab_glyph/trait.Font.html#glyph-layout-concepts
    pub line_gap: PhysicalPixelsF,
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
        let scale_physical: PhysicalPixels = scale.in_physical_scale(&scale_factor);
        //let scale = scale.in_device_pixels(&scale_factor);
        let atlas = Atlas::new(
            scale_physical.max(1024.into()),
            scale_physical.max(32.into()),
        );
        let scale: PhysicalPixelsF = scale_physical.into();
        // Init the font with PHYSICAL scale.
        let scaled = ab_font.as_scaled(scale.value());
        let height: PhysicalPixelsF = scaled.height().into();
        let line_gap: PhysicalPixelsF = scaled.line_gap().into();
        let ascent: PhysicalPixelsF = scaled.ascent().into();
        let descent: PhysicalPixelsF = scaled.descent().into();

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

        let scaled = self.ab_font.as_scaled(self.scale.value());
        let h_advance: PhysicalPixelsF = scaled.h_advance(id).into();

        let g = id.with_scale_and_position(
            self.scale.value(),
            ab_glyph::Point {
                x: 0.0,
                y: scaled.ascent(),
            },
        );

        let rect = {
            if let Some(g) = self.ab_font.outline_glyph(g) {
                let px_bounds = g.px_bounds();

                // TODO: Properly scale the bounds
                let px_width: PhysicalPixels = (px_bounds.width() as u32).into();
                let px_height: PhysicalPixels = (px_bounds.height() as u32).into();
                // base: the absolute position in the atlas where the glyph will be drawn.
                let (texture_bounds, atlas_width, pixels) = atlas.create_image(px_width, px_height);
                g.draw(|x, y, c| {
                    if c == 0.0 {
                        return; // Skip transparent pixels
                    }
                    let start = (texture_bounds.min.y + y) * atlas_width * 4
                        + (texture_bounds.min.x + x) * 4;
                    let start = start.value() as usize;
                    pixels[start] = ((1. - c) * 255.) as u8;
                    pixels[start + 1] = ((1. - c) * 255.) as u8;
                    pixels[start + 2] = ((1. - c) * 255.) as u8;
                    pixels[start + 3] = (c * 255.) as u8;
                });

                GlyphRect {
                    width: h_advance,
                    height: self.height,
                    bounds: PhysicalPixelsFRect {
                        min: PhysicalPixelsFPoint::new(
                            px_bounds.min.x.into(),
                            px_bounds.min.y.into(),
                        ),
                        max: PhysicalPixelsFPoint::new(
                            px_bounds.max.x.into(),
                            px_bounds.max.y.into(),
                        ),
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

    pub fn kern(&self, left: &Glyph, right: &Glyph) -> PhysicalPixelsF {
        let scaled = self.ab_font.as_scaled(self.scale.value());
        scaled.kern(left.id, right.id).into()
    }

    pub fn get_atlas_pixels(&self) -> Vec<u8> {
        self.atlas.lock().unwrap().pixels()
    }

    pub fn get_atlas_size(&self) -> PhysicalPixelsSize {
        let atlas = self.atlas.lock().unwrap();
        PhysicalPixelsSize::new(atlas.width, atlas.height)
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
                let new_font = Font::new(
                    ab_font.clone(),
                    font.name.clone(),
                    scale.into(),
                    self.scale_factor,
                );
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
    pub rect: PhysicalPixelsFRect,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TextAlignment {
    #[default]
    Left,
    Center,
    Right,
}

/// Composed glyphs, ready for rendering.
#[derive(Clone, Debug)]
pub struct Galley {
    pub glyphs: Vec<PositionedGlyph>,
    pub rect: LogicalPixelsRect,
}

#[derive(Clone, Debug)]
pub struct Typesetter {}

impl Typesetter {
    pub fn new() -> Self {
        Typesetter {}
    }

    /// Layouts a text
    pub fn compose(
        &self,
        text: &str,
        font: &Font,
        bounds: LogicalPixelsRect,
        _text_alignment: TextAlignment,
        alignment: Alignment,
        scale_factor: &ScaleFactor,
    ) -> Galley {
        let mut glyphs = Vec::new();
        let mut cursor = PhysicalPixelsFPoint::zero();
        let mut size: PhysicalPixelsFSize = PhysicalPixelsFSize::default();
        let mut last_glyph_id = None;
        for c in text.chars() {
            let glyph = font.get_glyph(c);
            if let Some(last) = last_glyph_id {
                cursor.x += font.kern(&last, &glyph);
            }
            if size.width < glyph.rect.height {
                size.height = glyph.rect.height;
            }
            glyphs.push(PositionedGlyph {
                glyph,
                rect: PhysicalPixelsFRect::with_size(
                    cursor,
                    PhysicalPixelsFSize::new(glyph.rect.width, glyph.rect.height),
                ),
            });
            last_glyph_id = Some(glyph);
            cursor.x += glyph.rect.width;
            size.width += glyph.rect.width;
        }

        Galley {
            glyphs,
            rect: alignment.align_size(bounds, size.in_logical_scale(&scale_factor)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zintl_render_math::{LogicalPixelsPoint, ScaleFactor};

    #[test]
    fn galley_logical_rect() {
        let scale_factor = ScaleFactor::new(1.0, 1.5);
        let font = Font::new(
            ab_glyph::FontArc::try_from_slice(include_bytes!(
                "../../../assets/inter/Inter-Regular.ttf"
            ))
            .unwrap(),
            "Inter".to_string(),
            16.0.into(),
            scale_factor,
        );
        let typesetter = Typesetter::new();
        let bounds = LogicalPixelsRect::new(
            LogicalPixelsPoint::new(0.0.into(), 0.0.into()),
            LogicalPixelsPoint::new(100.0.into(), 100.0.into()),
        );
        let galley = typesetter.compose(
            "Hello",
            &font,
            bounds,
            TextAlignment::Left,
            Alignment::TopLeft,
            &scale_factor,
        );
        assert_eq!(galley.rect.height(), 16.0.into());
        assert_eq!(galley.glyphs[0].glyph.rect.height, (16.0 * 1.5).into());
    }
}
