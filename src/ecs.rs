#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: glam::Vec3,
    pub rotation: f32,
    pub scale: glam::Vec3,
}

impl Transform {
    pub fn to_matrix(&self) -> glam::Mat4 {
        let translation = glam::Mat4::from_translation(self.position);
        let rotation = glam::Mat4::from_rotation_z(self.rotation);
        let scale = glam::Mat4::from_scale(self.scale);
        translation * rotation * scale
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub transform: [[f32; 4]; 4],
}
