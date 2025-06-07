use crate::scaling::{DevicePoint, DeviceRect};

/// Vertex in device pixels.
/// note: Texture bounds are not normalized.
#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: DevicePoint,
    pub tex_coords: DevicePoint,
}

/// Mesh in device pixels
#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub texture_id: Option<usize>,
    pub children: Vec<Mesh>,
}

impl Mesh {
    pub fn from_children(children: Vec<Mesh>) -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture_id: None,
            children,
        }
    }

    pub fn from_device_rect(
        rect: DeviceRect,
        texture_id: Option<usize>,
        tex_bounds: DeviceRect,
    ) -> Self {
        let vertices = vec![
            Vertex {
                position: rect.min,
                tex_coords: DevicePoint::new(tex_bounds.min.x, tex_bounds.min.y),
            },
            Vertex {
                position: DevicePoint::new(rect.max.x, rect.min.y),
                tex_coords: DevicePoint::new(tex_bounds.max.x, tex_bounds.min.y),
            },
            Vertex {
                position: DevicePoint::new(rect.max.x, rect.max.y),
                tex_coords: DevicePoint::new(tex_bounds.max.x, tex_bounds.max.y),
            },
            Vertex {
                position: DevicePoint::new(rect.min.x, rect.max.y),
                tex_coords: DevicePoint::new(tex_bounds.min.x, tex_bounds.max.y),
            },
        ];
        let indices = vec![0, 1, 2, 0, 2, 3];
        Self {
            vertices,
            indices,
            texture_id,
            children: Vec::new(),
        }
    }
}
