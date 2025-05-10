use zintl::render::RenderObject;

use crate::render::{Mesh, Vertex};

impl From<RenderObject> for Mesh {
    fn from(value: RenderObject) -> Self {
        match value.content {
            RenderContent::Shape(shape) => match shape {
                Shape::Rectangle => {
                    let vertices = vec![
                        Vertex {
                            position: [0.0, 0.0],
                            tex_coords: [0.0, 0.0],
                        },
                        Vertex {
                            position: [1.0, 0.0],
                            tex_coords: [1.0, 0.0],
                        },
                        Vertex {
                            position: [1.0, 1.0],
                            tex_coords: [1.0, 1.0],
                        },
                        Vertex {
                            position: [0.0, 1.0],
                            tex_coords: [0.0, 1.0],
                        },
                    ];
                    let indices = vec![0, 1, 2, 2, 3, 0];
                    Mesh {
                        vertices,
                        indices,
                        texture_id: 0,
                    }
                }
            },
            _ => Mesh::default(),
        }
    }
}
