use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::mesh::Rect;
use ab_glyph::Font;
use zintl_render_math::{Pixel, Point, Vec2};

/// All of the rendered glyphs in a single atlas.
#[derive(Clone, Debug)]
pub struct Atlas {
    /// A cached height of the atlas.
    height: usize,
    /// Where to draw new glyphs in the atlas.
    cursor: [usize; 2],
    /// The pixel data of the atlas, stored as a flat array of RGBA values.
    pixels: Vec<u8>,
    pixels_per_column: usize,
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
            pixels_per_column: initial_width,
            row_height: 0,
        }
    }

    fn resize_pixels(&mut self, new_height: usize) {
        if new_height > self.height {
            let new_size = self.pixels_per_column * new_height * 4;
            self.pixels.resize(new_size, 0);
            self.height = new_height;
        }
    }

    pub fn create_image(&mut self, width: usize, height: usize) -> (Point, &mut [u8]) {
        // Allocate a new texture with the specified width and height.
        {
            // We need to allocate a new row
            if self.cursor[0] + width > self.pixels_per_column {
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

        let part = self
            .pixels
            .get_mut(pos[0] * 4 * pos[1]..)
            .expect("new image may not be allocated");

        (
            Point {
                x: pos[0] as f32,
                y: pos[1] as f32,
            },
            part,
        )
    }

    /// Returns a reference to the pixel data of the atlas.
    pub fn pixels(&mut self) -> &mut [u8] {
        &mut self.pixels
    }
}

/// A single glyph data with size and coordinates.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Glyph {
    /// ab_glyph glyph id.
    pub id: ab_glyph::GlyphId,
    /// The width of the glyph in pixels.
    pub width: usize,
    pub rect: Rect,
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
    /// The offset for the font.
    pub offset: f32,
    /// A cached list of glyphs in the atlas.
    pub glyphs: HashMap<char, Glyph>,
}

impl Family {
    pub fn new(ab_font: ab_glyph::FontArc, family: String, scale: Pixel) -> Self {
        let atlas = Atlas::new(1024, 32);
        let offset = 0.0;

        Family {
            ab_font,
            family,
            atlas: Mutex::new(atlas).into(),
            scale,
            offset,
            glyphs: HashMap::new(),
        }
    }

    pub fn get_glyph(&self, c: char) -> Glyph {
        if let Some(glyph) = self.glyphs.get(&c) {
            return *glyph;
        }

        let atlas = &mut self.atlas.lock().unwrap();
        let id = self.ab_font.glyph_id(c);

        if id.0 == 0 {
            return Glyph::default();
        }

        let g = id.with_scale_and_position(self.scale, ab_glyph::point(0.0, 0.0));

        let (width, rect) = {
            if let Some(g) = self.ab_font.outline_glyph(g) {
                let bounds = g.px_bounds();
                let width = bounds.width() as usize;
                let height = bounds.height() as usize;
                let (point, pixels) = atlas.create_image(width, height);
                g.draw(|x, y, c| {
                    let x = x as usize;
                    let y = y as usize;
                    let base = (y * width * 4 + x * 4) as usize;
                    pixels[base] = ((1. - c) * 255.) as u8; // R
                    pixels[base + 1] = ((1. - c) * 255.) as u8;
                    pixels[base + 2] = ((1. - c) * 255.) as u8;
                    pixels[base + 3] = (c * 255.) as u8;
                });

                (
                    width,
                    Rect::new(Vec2::new(width as f32, height as f32), point),
                )
            } else {
                return Glyph::default();
            }
        };

        Glyph { id, width, rect }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FamilyProperties {
    pub name: String,
    pub size: f32,
}

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
        let family = ab_glyph::FontArc::try_from_slice(&data).unwrap();
        self.families.insert(name.clone(), family.clone());
    }

    pub fn get_family(&mut self, family: FamilyProperties) -> Option<&Family> {
        match self.sized_families.get(&family) {
            Some(f) => Some(f),
            None => {
                if let Some(ab_font) = self.families.get(&family.name) {
                    let scale = Pixel::new(family.size);
                    let new_family = Family::new(ab_font.clone(), family.name.clone(), scale);
                    self.sized_families.insert(family.clone(), new_family);
                    self.sized_families.get(&family)
                } else {
                    None
                }
            }
        }
    }
}

pub struct TextTessellator {
    pub font_manager: Arc<FamilyManager>,
}

impl TextTessellator {
    pub fn new(font_manager: Arc<FamilyManager>) -> Self {
        TextTesselator { font_manager }
    }
}
