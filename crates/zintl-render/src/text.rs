use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::scaling::{
    DevicePixels, DevicePoint, DeviceRect, DeviceSize, LogicalPixels, LogicalPoint, LogicalRect,
};
use ab_glyph::{Font as _, ScaleFont};
use zintl_render_math::Vec2;

/// All of the rendered glyphs in a single atlas.
#[derive(Clone, Debug)]
pub struct Atlas {
    /// A cached height of the atlas.
    height: DevicePixels,
    /// Where to draw new glyphs in the atlas.
    cursor: DevicePoint,
    /// The pixel data of the atlas, stored as a flat array of RGBA values.
    pixels: Vec<u8>,
    width: DevicePixels,
    row_height: DevicePixels,
}

impl Atlas {
    /// Creates a new `Atlas` with the specified width and height.
    pub fn new(initial_width: DevicePixels, initial_height: DevicePixels) -> Self {
        let pixels = vec![0; (initial_width * initial_height * 4) as usize];
        Atlas {
            height: initial_height,
            cursor: DevicePoint::new(0, 0),
            pixels,
            width: initial_width,
            row_height: 0,
        }
    }

    fn resize_pixels(&mut self, new_height: DevicePixels) {
        if new_height > self.height {
            let new_size = self.width * new_height * 4;
            self.pixels.resize(new_size as usize, 0);
            self.height = new_height;
        }
    }

    /// Texture bounds (is not normalized), atlas width, and mutable pixel data.
    pub fn create_image(
        &mut self,
        width: DevicePixels,
        height: DevicePixels,
    ) -> (DeviceRect, DevicePixels, &mut Vec<u8>) {
        // Allocate a new texture with the specified width and height.
        {
            // We need to allocate a new row
            if self.cursor.x + width > self.width {
                self.cursor.x = 0;
                self.cursor.y += self.row_height;
                self.row_height = 0;
            }

            self.row_height = self.row_height.max(height);

            let new_height = self.cursor.y + self.row_height;
            self.resize_pixels(new_height);
        }

        let pos = self.cursor;
        self.cursor.x += width;

        (
            DeviceRect::new(pos, DevicePoint::new(pos.x + width, pos.y + height)),
            self.width.clone(),
            &mut self.pixels,
        )
    }

    /// Returns a reference to the pixel data of the atlas.
    pub fn pixels_mut_ref(&mut self) -> &mut [u8] {
        &mut self.pixels
    }

    pub fn pixels(&self) -> Vec<u8> {
        self.pixels.clone()
    }
}

/// A rectangle and texture coordinates for a glyph.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GlyphRect {
    /// The width of the glyph in pixels.
    pub width: LogicalPixels,
    /// The height of the glyph in pixels.
    pub height: LogicalPixels,
    /// Mesh bounds on the glyph rectangle. NOT FOR LAYOUTING.
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
    pub scale: LogicalPixels,
    pub height: LogicalPixels,
    /// https://docs.rs/ab_glyph/latest/ab_glyph/trait.Font.html#glyph-layout-concepts
    pub ascent: LogicalPixels,
    /// https://docs.rs/ab_glyph/latest/ab_glyph/trait.Font.html#glyph-layout-concepts
    pub descent: LogicalPixels,
    /// https://docs.rs/ab_glyph/latest/ab_glyph/trait.Font.html#glyph-layout-concepts
    pub line_gap: LogicalPixels,
    /// A cached list of glyphs in the atlas.
    pub glyphs: Arc<Mutex<HashMap<char, Glyph>>>,
}

impl Font {
    pub fn new(ab_font: ab_glyph::FontArc, type_face: String, scale: LogicalPixels) -> Self {
        let atlas = Atlas::new(128, 32);
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
        let scaled = self.ab_font.as_scaled(self.scale);
        let g = scaled.scaled_glyph(c);
        let id = g.id;

        if id.0 == 0 {
            return Glyph::default();
        }

        let advance = scaled.h_advance(id);

        let rect = {
            if let Some(g) = self.ab_font.outline_glyph(g) {
                let px_bounds = g.px_bounds();
                // TODO: Properly scale the bounds
                let px_width = px_bounds.width() as DevicePixels;
                let px_height = px_bounds.height() as DevicePixels;
                // base: the absolute position in the atlas where the glyph will be drawn.
                let (texture_bounds, atlas_width, pixels) = atlas.create_image(px_width, px_height);

                g.draw(|x, y, c| {
                    if c < 0.01 {
                        return; // Skip transparent pixels
                    }
                    let start = (texture_bounds.min.y + y) * atlas_width * 4
                        + (texture_bounds.min.x + x) * 4;
                    let start = start as usize;
                    pixels[start] = ((1. - c) * 255.) as u8; // R
                    pixels[start + 1] = ((1. - c) * 255.) as u8;
                    pixels[start + 2] = ((1. - c) * 255.) as u8;
                    pixels[start + 3] = (c * 255.) as u8;
                });

                GlyphRect {
                    width: advance,
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
}

impl Typecase {
    pub fn new() -> Self {
        Typecase {
            fonts: HashMap::new(),
            sized_fonts: HashMap::new(),
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
                let new_font = Font::new(ab_font.clone(), font.name.clone(), scale);
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

impl Galley {
    /*pub fn to_meshes(&self, texture_size: DeviceSize) -> Vec<Mesh> {
        self.to_meshes_with_texture_id(0, texture_size)
    }

    pub fn to_meshes_with_texture_id(
        &self,
        texture_id: usize,
        texture_size: DeviceSize,
    ) -> Vec<Mesh> {
        let mut meshes = Vec::new();
        for positioned_glyph in &self.glyphs {
            let layout_rect = positioned_glyph.rect;
            let glyph = &positioned_glyph.glyph;
            let mesh_bounds = glyph.rect.bounds;

            let mesh_rect = Rect {
                min: Point::new(
                    layout_rect.min.x + mesh_bounds.min.x,
                    layout_rect.max.y + mesh_bounds.min.y,
                ),
                max: Point::new(
                    layout_rect.min.x + mesh_bounds.max.x,
                    layout_rect.max.y + mesh_bounds.max.y,
                ),
            };
            let tex_coords = glyph.rect.texture_coords.normalize(texture_size);
            let mesh = mesh_rect.to_mesh(texture_id, tex_coords);

            meshes.push(mesh);
        }
        meshes
    }

    /// For debugging porposes.
    pub fn to_meshes_from_layout_rect(&self, texture_id: usize, texture_size: Vec2) -> Vec<Mesh> {
        let mut meshes = Vec::new();
        for positioned_glyph in &self.glyphs {
            let layout_rect = positioned_glyph.rect;
            let tex_coords = positioned_glyph
                .glyph
                .rect
                .texture_coords
                .normalize(texture_size);
            let mesh = layout_rect.to_mesh(texture_id, tex_coords);

            meshes.push(mesh);
        }
        meshes
    }*/
}

#[derive(Clone, Debug)]
pub struct Typesetter {}

impl Typesetter {
    pub fn new() -> Self {
        Typesetter {}
    }

    /// Layouts a text
    pub fn compose(&self, text: &str, font: &Font, position: LogicalPoint) -> Galley {
        let position = LogicalPoint::new(position.x, position.y + font.ascent);
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
