use crate::mesh::Mesh;
use crate::text::Galley;

use zintl_render_math::{PhysicalPixelsPoint, PhysicalPixelsRect, Viewport};

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

    pub fn tessellate_galley(&mut self, galley: &Galley) -> Vec<Mesh> {
        let mut meshes = Vec::new();
        for positioned_glyph in &galley.glyphs {
            let layout_rect = positioned_glyph.rect;
            let glyph = &positioned_glyph.glyph;
            let mesh_bounds = glyph.rect.bounds;

            // TODO: + operator
            let mesh_rect = PhysicalPixelsRect {
                min: PhysicalPixelsPoint::new(
                    (layout_rect.min.x + mesh_bounds.min.x).into(),
                    (layout_rect.min.y + mesh_bounds.min.y).into(),
                ),
                max: PhysicalPixelsPoint::new(
                    (layout_rect.min.x + mesh_bounds.max.x).into(),
                    (layout_rect.min.y + mesh_bounds.max.y).into(),
                ),
            };

            let mesh = Mesh::from_device_rect(mesh_rect, Some(0), glyph.rect.texture_bounds);

            meshes.push(mesh);
        }
        meshes
    }

    pub fn tessellate(&mut self, job: &TessellationJob, _viewport: &Viewport) -> Vec<Mesh> {
        match job {
            TessellationJob::Galley(galley) => self.tessellate_galley(galley),
            _ => {
                vec![]
            }
        }
    }
}
