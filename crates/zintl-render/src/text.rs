use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::mesh::{Mesh, Rect};
use ab_glyph::{Font, ScaleFont};
use zintl_render_math::{Pixel, Point, PointUSize, Vec2};

/// All of the rendered glyphs in a single atlas.
#[derive(Clone, Debug)]
pub struct Atlas {
    /// A cached height of the atlas.
    height: usize,
    /// Where to draw new glyphs in the atlas.
    cursor: [usize; 2],
    /// The pixel data of the atlas, stored as a flat array of RGBA values.
    pixels: Vec<u8>,
    width: usize,
    row_height: usize,
}

impl Atlas {
    /// Creates a new `Atlas` with the specified width and height.
    pub fn new(initial_width: usize, initial_height: usize) -> Self {
        let pixels = vec![0; initial_width * initial_height * 4];
        Atlas {
            height: initial_height,
            cursor: [0, 0],
            pixels,
            width: initial_width,
            row_height: 0,
        }
    }

    fn resize_pixels(&mut self, new_height: usize) {
        if new_height > self.height {
            let new_size = self.width * new_height * 4;
            self.pixels.resize(new_size, 0);
            self.height = new_height;
        }
    }

    /// Texture min coords, max coords, atlas width, and mutable pixel data.
    pub fn create_image(
        &mut self,
        width: usize,
        height: usize,
    ) -> (PointUSize, PointUSize, usize, &mut Vec<u8>) {
        // Allocate a new texture with the specified width and height.
        {
            // We need to allocate a new row
            if self.cursor[0] + width > self.width {
                self.cursor[0] = 0;
                self.cursor[1] += self.row_height;
                self.row_height = 0;
            }

            self.row_height = self.row_height.max(height);

            let new_height = self.cursor[1] + self.row_height;
            self.resize_pixels(new_height);
        }

        let pos = self.cursor;
        self.cursor[0] += width;

        println!("{}x{} at ({}, {})", width, height, pos[0], pos[1]);

        (
            PointUSize::new(pos[0], pos[1]),
            PointUSize::new(pos[0] + width, pos[1] + height),
            self.width.clone(),
            &mut self.pixels,
        )
    }

    pub fn normalize_coords(&self, point: PointUSize) -> Point {
        let x = point.x as f32 / self.width as f32;
        let y = point.y as f32 / self.height as f32;
        println!(
            "Normalizing coords: ({}, {}) -> ({}, {})",
            point.x, point.y, x, y
        );
        Point::new(x, y)
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
    /// The horizontal offset from previous glyph.
    pub x_offset: Pixel,
    /// The vertical offset of the glyph. Also known as the line gap.
    pub y_offset: Pixel,
    /// The width of the glyph in pixels.
    pub width: Pixel,
    /// The height of the glyph in pixels.
    pub height: Pixel,
    /// Mesh bounds on the glyph rectangle. NOT FOR LAYOUTING.
    pub bounds: Rect,
    /// The texture coordinates of the glyph in the atlas.
    pub texture_coords: Rect,
}

/// A single glyph data with size and coordinates.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Glyph {
    /// ab_glyph glyph id.
    pub id: ab_glyph::GlyphId,
    pub rect: GlyphRect,
}

/// A font family with a specific size.
#[derive(Clone, Debug)]
pub struct Family {
    pub ab_font: ab_glyph::FontArc,
    pub family: String,
    /// The atlas containing the rendered glyphs.
    pub atlas: Arc<Mutex<Atlas>>,
    /// The scale factor for the font.
    pub scale: Pixel,
    pub height: f32,
    /// https://docs.rs/ab_glyph/latest/ab_glyph/trait.Font.html#glyph-layout-concepts
    pub ascent: f32,
    /// https://docs.rs/ab_glyph/latest/ab_glyph/trait.Font.html#glyph-layout-concepts
    pub descent: f32,
    /// https://docs.rs/ab_glyph/latest/ab_glyph/trait.Font.html#glyph-layout-concepts
    pub line_gap: f32,
    /// A cached list of glyphs in the atlas.
    pub glyphs: Arc<Mutex<HashMap<char, Glyph>>>,
}

impl Family {
    pub fn new(ab_font: ab_glyph::FontArc, family: String, scale: Pixel) -> Self {
        let atlas = Atlas::new(128, 32);
        let scaled = ab_font.as_scaled(scale);
        let height = scaled.height();
        let line_gap = scaled.line_gap();
        let ascent = scaled.ascent();
        let descent = scaled.descent();

        Family {
            ab_font,
            family,
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
        let id = self.ab_font.glyph_id(c);

        if id.0 == 0 {
            return Glyph::default();
        }

        println!("scale: {}", self.scale);

        let g = id.with_scale_and_position(self.scale, ab_glyph::point(0.0, 0.0));
        let scaled = self.ab_font.as_scaled(self.scale);
        let advance = scaled.h_advance(id);
        let side_bearing = scaled.h_side_bearing(id);

        let rect = {
            if let Some(g) = self.ab_font.outline_glyph(g) {
                let px_bounds = g.px_bounds();
                let px_width = px_bounds.width() as usize;
                let px_height = px_bounds.height() as usize;
                // base: the absolute position in the atlas where the glyph will be drawn.
                let (atlas_min, atlas_max, atlas_width, pixels) =
                    atlas.create_image(px_width, px_height);

                g.draw(|x, y, c| {
                    let x = x as usize;
                    let y = y as usize;
                    let start = (atlas_min.y + y) * atlas_width * 4 + (atlas_min.x + x) * 4;
                    pixels[start] = ((1. - c) * 255.) as u8; // R
                    pixels[start + 1] = ((1. - c) * 255.) as u8;
                    pixels[start + 2] = ((1. - c) * 255.) as u8;
                    pixels[start + 3] = (c * 255.) as u8;
                });

                GlyphRect {
                    x_offset: side_bearing,
                    y_offset: self.line_gap,
                    width: advance,
                    height: self.height,
                    bounds: Rect {
                        min: Point::new(px_bounds.min.x, px_bounds.min.y),
                        max: Point::new(px_bounds.max.x, px_bounds.max.y),
                    },
                    texture_coords: Rect::new(atlas_min.into(), atlas_max.into()),
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

    pub fn get_atlas_pixels(&self) -> Vec<u8> {
        self.atlas.lock().unwrap().pixels()
    }

    pub fn get_atlas_size(&self) -> (usize, usize) {
        let atlas = self.atlas.lock().unwrap();
        (atlas.width, atlas.height)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Hash, Eq)]
pub struct FamilyProperties {
    pub name: String,
    /// f32 does not implement Hash, so we use a String instead.
    pub scale_string: String,
}

#[derive(Clone, Debug)]
pub struct FamilyManager {
    pub families: HashMap<String, ab_glyph::FontArc>,
    pub sized_families: HashMap<FamilyProperties, Family>,
}

impl FamilyManager {
    pub fn new() -> Self {
        FamilyManager {
            families: HashMap::new(),
            sized_families: HashMap::new(),
        }
    }

    pub fn load_family(&mut self, name: String, data: Vec<u8>) {
        let family = ab_glyph::FontVec::try_from_vec(data)
            .map(ab_glyph::FontArc::from)
            .unwrap();
        self.families.insert(name.clone(), family.clone());
    }

    pub fn get_family(&mut self, family: FamilyProperties) -> Option<&Family> {
        let f = self.sized_families.entry(family.clone()).or_insert({
            if let Some(ab_font) = self.families.get(&family.name) {
                let scale = family
                    .scale_string
                    .parse::<f32>()
                    .expect("Invalid scale string");
                let new_family = Family::new(ab_font.clone(), family.name.clone(), scale);
                new_family
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
    pub rect: Rect,
}

/// Composed glyphs, ready for rendering.
#[derive(Clone, Debug)]
pub struct Galley {
    pub glyphs: Vec<PositionedGlyph>,
    pub rect: Rect,
}

impl Galley {
    pub fn to_meshes(&self, texture_size: Vec2) -> Vec<Mesh> {
        let mut meshes = Vec::new();
        for positioned_glyph in &self.glyphs {
            let layout_rect = positioned_glyph.rect;
            let glyph = &positioned_glyph.glyph;
            let mesh_bounds = glyph.rect.bounds;

            let mesh_rect = Rect {
                min: Point::new(
                    layout_rect.min.x + mesh_bounds.min.x,
                    layout_rect.min.y + mesh_bounds.min.y,
                ),
                max: Point::new(
                    layout_rect.min.x + mesh_bounds.max.x,
                    layout_rect.min.y + mesh_bounds.max.y,
                ),
            };
            let tex_coords = glyph.rect.texture_coords.normalize(texture_size);
            let mesh = mesh_rect.to_mesh(
                0, // TODO: Specific proper texture_id
                tex_coords,
            );

            meshes.push(mesh);
        }
        meshes
    }
}

#[derive(Clone, Debug)]
pub struct Typesetter {}

impl Typesetter {
    pub fn new() -> Self {
        Typesetter {}
    }

    pub fn compose(&self, text: &str, family: &Family, position: Point) -> Galley {
        let mut glyphs = Vec::new();
        let mut cursor = position.clone();
        let mut size: Vec2 = Vec2::default();
        for c in text.chars() {
            let glyph = family.get_glyph(c);
            cursor.x += glyph.rect.width + glyph.rect.x_offset;
            size.x += glyph.rect.width + glyph.rect.x_offset;
            if size.y < glyph.rect.height {
                size.y = glyph.rect.height;
            }
            glyphs.push(PositionedGlyph {
                glyph,
                rect: Rect::with_size(cursor, Vec2::new(glyph.rect.width, glyph.rect.height)),
            });
        }
        Galley {
            glyphs,
            rect: Rect::with_size(position, size),
        }
    }
}
