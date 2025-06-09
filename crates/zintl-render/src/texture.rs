use crate::scaling::{DevicePixels, DevicePoint, DeviceRect};

/// All of the rendered glyphs in a single atlas.
#[derive(Clone, Debug)]
pub struct Atlas {
    /// A cached width of the atlas.
    pub width: DevicePixels,
    /// A cached height of the atlas.
    pub height: DevicePixels,
    /// Where to draw new glyphs in the atlas.
    cursor: DevicePoint,
    /// The pixel data of the atlas, stored as a flat array of RGBA values.
    pixels: Vec<u8>,
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

    pub fn resize_pixels(&mut self, new_height: DevicePixels) {
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
