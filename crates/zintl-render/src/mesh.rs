use zintl_render_math::{Mat4, Point, Vec2};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
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
#[derive(Clone, Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub texture_id: Option<usize>,
    pub children: Vec<Mesh>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    /// The size on the screen
    pub size: Vec2,
    /// The top-left coordinate of the texture
    pub tex_coords: Point,
}

impl Rect {
    pub fn new(size: Vec2, tex_coords: Point) -> Self {
        Self { size, tex_coords }
    }

    pub fn from_size(size: Vec2) -> Self {
        Self::new(size, Point::new(0.0, 0.0))
    }

    pub fn from_tex_coords(tex_coords: Point) -> Self {
        Self::new(Vec2::new(0., 0.), tex_coords)
    }

    pub fn to_mesh(&self, coords: Point, texture_id: usize) -> Mesh {
        Mesh {
            vertices: vec![
                Vertex {
                    position: Point::new(coords.x, coords.y),
                    tex_coords: Point::new(self.tex_coords.x, self.tex_coords.y),
                },
                Vertex {
                    position: Point::new(coords.x + self.size.x, coords.y),
                    tex_coords: Point::new(self.tex_coords.x + self.size.x, self.tex_coords.y),
                },
                Vertex {
                    position: Point::new(coords.x + self.size.x, coords.y + self.size.y),
                    tex_coords: Point::new(
                        self.tex_coords.x + self.size.x,
                        self.tex_coords.y + self.size.y,
                    ),
                },
                Vertex {
                    position: Point::new(coords.x, coords.y + self.size.y),
                    tex_coords: Point::new(self.tex_coords.x, self.tex_coords.y + self.size.y),
                },
            ],
            indices: vec![0, 1, 2, 0, 2, 3],
            texture_id: Some(texture_id),
            children: Vec::new(),
        }
    }
}
