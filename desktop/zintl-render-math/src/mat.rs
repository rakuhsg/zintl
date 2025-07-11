/// Matrix 3x3
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Mat3 {
    pub m: [[f32; 3]; 3],
}

/// Matrix 4x4
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Mat4 {
    pub m: [[f32; 4]; 4],
}

impl From<Mat4> for cgmath::Matrix4<f32> {
    fn from(mat: Mat4) -> Self {
        cgmath::Matrix4::from(mat.m)
    }
}

impl From<cgmath::Matrix4<f32>> for Mat4 {
    fn from(mat: cgmath::Matrix4<f32>) -> Self {
        Mat4 { m: mat.into() }
    }
}
