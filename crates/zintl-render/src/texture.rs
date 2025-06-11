use zintl_render_math::{PhysicalPixels, PhysicalPixelsPoint, PhysicalPixelsRect};

/// All of the rendered glyphs in a single atlas.
#[derive(Clone, Debug)]
pub struct Atlas {
    /// A cached width of the atlas.
    pub width: PhysicalPixels,
    /// A cached height of the atlas.
    pub height: PhysicalPixels,
    /// Where to draw new glyphs in the atlas.
    cursor: PhysicalPixelsPoint,
    /// The pixel data of the atlas, stored as a flat array of RGBA values.
    pixels: Vec<u8>,
    row_height: PhysicalPixels,
}

impl Atlas {
    /// Creates a new `Atlas` with the specified width and height.
    pub fn new(initial_width: PhysicalPixels, initial_height: PhysicalPixels) -> Self {
        let pixels = vec![0; (initial_width * initial_height * 4).value() as usize];
        Atlas {
            height: initial_height,
            cursor: PhysicalPixelsPoint::new(0.into(), 0.into()),
            pixels,
            width: initial_width,
            row_height: PhysicalPixels::zero(),
        }
    }

    pub fn resize_pixels(&mut self, new_height: PhysicalPixels) {
        if new_height > self.height {
            let new_size = self.width * new_height * 4;
            self.pixels.resize(new_size.value() as usize, 0);
            self.height = new_height;
        }
    }

    /// Texture bounds (is not normalized), atlas width, and mutable pixel data.
    pub fn create_image(
        &mut self,
        width: PhysicalPixels,
        height: PhysicalPixels,
    ) -> (PhysicalPixelsRect, PhysicalPixels, &mut Vec<u8>) {
        // Allocate a new texture with the specified width and height.
        {
            // We need to allocate a new row
            if self.cursor.x + width > self.width {
                self.cursor.x = 0.into();
                self.cursor.y += self.row_height;
                self.row_height = 0.into();
            }

            self.row_height = self.row_height.max(height);

            let new_height = self.cursor.y + self.row_height;
            self.resize_pixels(new_height);
        }

        let pos = self.cursor;
        self.cursor.x += width;

        (
            PhysicalPixelsRect::new(pos, PhysicalPixelsPoint::new(pos.x + width, pos.y + height)),
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
