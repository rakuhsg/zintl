use zintl_render_math::{Mat4, Point, Vec2};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: Point,
    pub tex_coords: Point,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub ortho: Mat4,
}

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
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    pub fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    pub fn with_size(min: Point, size: Vec2) -> Self {
        let max = Point::new(min.x + size.x, min.y + size.y);
        Self { min, max }
    }

    pub fn zero() -> Self {
        Self {
            min: Point::new(0.0, 0.0),
            max: Point::new(0.0, 0.0),
        }
    }

    pub fn normalize(&self, size: Vec2) -> Self {
        let min = Point::new(self.min.x / size.x, self.min.y / size.y);
        let max = Point::new(self.max.x / size.x, self.max.y / size.y);
        Self { min, max }
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    /// `tex_coords` is must be in the range [0.0, 1.0]
    pub fn to_mesh(&self, texture_id: usize, tex_coords: Rect) -> Mesh {
        Mesh {
            vertices: vec![
                Vertex {
                    position: self.min,
                    tex_coords: tex_coords.min,
                },
                Vertex {
                    position: Point::new(self.max.x, self.min.y),
                    tex_coords: Point::new(tex_coords.max.x, tex_coords.min.y),
                },
                Vertex {
                    position: self.max,
                    tex_coords: tex_coords.max,
                },
                Vertex {
                    position: Point::new(self.min.x, self.max.y),
                    tex_coords: Point::new(tex_coords.min.x, tex_coords.max.y),
                },
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
            texture_id: Some(texture_id),
            children: Vec::new(),
        }
    }
    // TODO: Remove this
    /*    pub fn to_mesh(&self, coords: Point, texture_id: usize, texture_size: Vec2) -> Mesh {
        // Normalize texture coordinates based on the texture size
        let tex_min = Point::new(
            self.tex_min.x / texture_size.x,
            self.tex_min.y / texture_size.y,
        );
        let tex_max = Point::new(
            self.tex_max.x / texture_size.x,
            self.tex_max.y / texture_size.y,
        );
        Mesh {
            vertices: vec![
                Vertex {
                    position: Point::new(coords.x, coords.y),
                    tex_coords: Point::new(tex_min.x, tex_min.y),
                },
                Vertex {
                    position: Point::new(coords.x + self.size.x, coords.y),
                    tex_coords: Point::new(tex_max.x, tex_min.y),
                },
                Vertex {
                    position: Point::new(coords.x + self.size.x, coords.y + self.size.y),
                    tex_coords: Point::new(tex_max.x, tex_max.y),
                },
                Vertex {
                    position: Point::new(coords.x, coords.y + self.size.y),
                    tex_coords: Point::new(tex_min.x, tex_max.y),
                },
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
            texture_id: Some(texture_id),
            children: Vec::new(),
        }
    }*/
}
