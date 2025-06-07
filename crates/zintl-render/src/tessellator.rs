use crate::mesh::Mesh;
use crate::scaling::{DevicePoint, DeviceRect, LogicalPoint, LogicalRect, Viewport};
use crate::text::Galley;

pub enum TessellationJob {
    Galley(Galley),
    #[allow(dead_code)]
    Empty,
}

#[derive(Clone, Debug)]
pub struct Tessellator {}

impl Tessellator {
    pub fn new() -> Self {
        Tessellator {}
    }

    pub fn tessellate_galley(&mut self, galley: &Galley, viewport: &Viewport) -> Vec<Mesh> {
        let device_pixel_ratio = viewport.scale_factor.device_pixel_ratio;
        let mut meshes = Vec::new();
        for positioned_glyph in &galley.glyphs {
            let layout_rect = positioned_glyph.rect;
            let glyph = &positioned_glyph.glyph;
            let mesh_bounds = glyph.rect.bounds;

            // TODO: + operator
            let mesh_rect = DeviceRect {
                min: LogicalPoint::new(
                    layout_rect.min.x + mesh_bounds.min.x,
                    layout_rect.max.y + mesh_bounds.min.y,
                )
                .scale(device_pixel_ratio),
                max: LogicalPoint::new(
                    layout_rect.min.x + mesh_bounds.max.x,
                    layout_rect.max.y + mesh_bounds.max.y,
                )
                .scale(device_pixel_ratio),
            };
            let mesh = Mesh::from_device_rect(mesh_rect, Some(0), glyph.rect.texture_bounds);

            meshes.push(mesh);
        }
        meshes
    }

    pub fn tessellate(&mut self, job: &TessellationJob, viewport: &Viewport) -> Vec<Mesh> {
        match job {
            TessellationJob::Galley(galley) => self.tessellate_galley(galley, viewport),
            _ => {
                vec![]
            }
        }
    }
}
