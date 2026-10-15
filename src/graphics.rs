use std::sync::Arc;

use hecs::Entity;
use wgpu::CurrentSurfaceTexture;
use winit::window::Window;

#[derive(Default)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

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

pub struct GpuMeshInstance {
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

pub struct PlayerTag;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub transform: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
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
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Arc<Window>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    _uniform_bind_group_layout: wgpu::BindGroupLayout,
    world: hecs::World,
    pub input: InputState,
}

impl State {
    pub fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance =
            wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
            ..Default::default()
        }))
        .expect("no adapter");

        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::defaults(),
            label: None,
            ..Default::default()
        }))
        .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Auto,
        };
        surface.configure(&device, &config);

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Uniform bind group layout"),
            });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader with Uniforms"),
            source: wgpu::ShaderSource::Wgsl(
                r#"
            struct Uniforms {
                transform: mat4x4<f32>,
            };
            @group(0) @binding(0) var<uniform> uniforms: Uniforms;

            struct VertexInput {
                @location(0) position: vec3<f32>,
                @location(1) color: vec3<f32>,
            };

            struct VertexOutput {
                @builtin(position) clip_position: vec4<f32>,
                @location(0) color: vec3<f32>,
            };

            @vertex
            fn vs_main(model: VertexInput) -> VertexOutput {
                var out: VertexOutput;
                out.color = model.color;
                out.clip_position = uniforms.transform * vec4<f32>(model.position, 1.0);
                return out;
            }

            @fragment
            fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
                return vec4<f32>(in.color, 1.0);
            }
            "#
                .into(),
            ),
        });

        use wgpu::util::DeviceExt;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = VERTICES.len() as u32;

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render pipeline layout"),
                bind_group_layouts: &[Some(&uniform_bind_group_layout)],
                ..Default::default()
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(Vertex::desc())],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        let mut world = hecs::World::new();

        let spawn_mesh = |device: &wgpu::Device, layout: &wgpu::BindGroupLayout| {
            let uniform_data = Uniforms {
                transform: glam::Mat4::IDENTITY.to_cols_array_2d(),
            };

            let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform buffer"),
                contents: bytemuck::cast_slice(&[uniform_data]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Uniform binding group"),
                layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }],
            });
            GpuMeshInstance {
                uniform_buffer,
                bind_group,
            }
        };

        world.spawn((
            PlayerTag,
            Transform {
                position: glam::Vec3::new(-0.4, 0.0, 0.0),
                rotation: 0.0,
                scale: glam::Vec3::splat(0.5),
            },
            spawn_mesh(&device, &uniform_bind_group_layout),
        ));

        world.spawn((
            Transform {
                position: glam::Vec3::new(0.4, 0.0, 0.0),
                rotation: 0.0,
                scale: glam::Vec3::splat(0.5),
            },
            spawn_mesh(&device, &uniform_bind_group_layout),
        ));

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
            vertex_buffer,
            num_vertices,
            _uniform_bind_group_layout: uniform_bind_group_layout,
            world,
            input: InputState::default(),
        }
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn update(&mut self, dt: f32) {
        let speed = 1.5;
        let mut move_dir = glam::Vec3::ZERO;
        if self.input.up {
            move_dir.y += 1.0;
        }
        if self.input.down {
            move_dir.y -= 1.0;
        }
        if self.input.left {
            move_dir.x -= 1.0;
        }
        if self.input.right {
            move_dir.x += 1.0;
        }

        for (_, _tag, transform) in &mut self.world.query::<(Entity, &PlayerTag, &mut Transform)>()
        {
            transform.position += move_dir.normalize_or_zero() * speed * dt;
        }

        for (entity, transform) in &mut self.world.query::<(Entity, &mut Transform)>() {
            if self.world.get::<&PlayerTag>(entity).is_err() {
                transform.rotation -= 2.0 * dt;
            }
        }

        for (_, transform, gpu) in &mut self.world.query::<(Entity, &Transform, &GpuMeshInstance)>()
        {
            let uniform_data = Uniforms {
                transform: transform.to_matrix().to_cols_array_2d(),
            };
            self.queue.write_buffer(
                &gpu.uniform_buffer,
                0,
                bytemuck::cast_slice(&[uniform_data]),
            );
        }
    }

    pub fn render(&self) -> bool {
        if let CurrentSurfaceTexture::Success(output) = self.surface.get_current_texture() {
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render encoder"),
                });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.05,
                                g: 0.05,
                                b: 0.1,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                    ..Default::default()
                });

                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

                for (_, gpu) in &mut self.world.query::<(Entity, &GpuMeshInstance)>() {
                    render_pass.set_bind_group(0, &gpu.bind_group, &[]);
                    render_pass.draw(0..self.num_vertices, 0..1);
                }
            }
            self.queue.submit(std::iter::once(encoder.finish()));
            self.queue.present(output);
            return true;
        }
        false
    }

    pub(crate) fn window_id(&self) -> winit::window::WindowId {
        self.window.id()
    }
}
