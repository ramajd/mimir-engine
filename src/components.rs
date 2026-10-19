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

pub struct PlayerTag;

#[derive(Debug, Clone, Copy)]
pub struct Collider2D {
    pub half_size: glam::Vec2,
}

impl Collider2D {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            half_size: glam::Vec2::new(width * 0.5, height * 0.5),
        }
    }

    pub fn intersects(&self, pos_a: glam::Vec2, other: &Collider2D, pos_b: glam::Vec2) -> bool {
        let delta = (pos_a - pos_b).abs();
        let total_half_size = self.half_size + other.half_size;

        delta.x < total_half_size.x && delta.y < total_half_size.y
    }
}

pub struct GpuMeshInstance {
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub mvp: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.3, 0.3, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-0.3, -0.3, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.3, -0.3, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [-0.3, 0.3, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [0.3, -0.3, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.3, 0.3, 0.0],
        tex_coords: [1.0, 0.0],
    },
];
