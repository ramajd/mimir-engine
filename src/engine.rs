use std::sync::Arc;

use hecs::Entity;
use wgpu::util::DeviceExt;
use wgpu::{CurrentSurfaceTexture, TexelCopyBufferLayout};
use winit::event::KeyEvent;
use winit::window::Window;

use crate::audio::AudioManager;
use crate::camera::Camera2D;
use crate::components::{
    Collider2D, GpuMeshInstance, PlayerTag, Transform, Uniforms, VERTICES, Vertex,
};
use crate::input::InputState;
use crate::scenes::game::GamePlayState;
use crate::scenes::menu::MainMenuState;
use crate::state::{AppState, StateMachine};

pub struct EngineState<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Arc<Window>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    pub input: InputState,
    audio_manager: AudioManager,
    current_state: AppState,
    menu_scene: MainMenuState,
    game_scene: GamePlayState,
    diffuse_texture_view: wgpu::TextureView,
    diffuse_sampler: wgpu::Sampler,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl<'a> EngineState<'a> {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let mut audio_manager = AudioManager::new().unwrap_or_else(|e| {
            eprintln!("Warning: Failed to initialize audio output: {}", e);
            panic!("Audio system failed to initialize");
        });
        audio_manager.play_background_music("background.wav");

        let instance =
            wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                ..Default::default()
            })
            .await
            .expect("no adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::defaults(),
                label: None,
                ..Default::default()
            })
            .await
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

        let img_width = 256;
        let img_height = 256;
        let mut img_data = vec![0u8; (img_width * img_height * 4) as usize];
        for y in 0..img_height {
            for x in 0..img_width {
                let idx = ((y * img_width + x) * 4) as usize;
                let checker = ((x / 32) + (y / 32)) % 2 == 0;
                if checker {
                    img_data[idx] = 255;
                    img_data[idx + 1] = 255;
                    img_data[idx + 2] = 255;
                } else {
                    img_data[idx] = 50;
                    img_data[idx + 1] = 50;
                    img_data[idx + 2] = 150;
                }
                img_data[idx + 3] = 255;
            }
        }

        let texture_size = wgpu::Extent3d {
            width: img_width,
            height: img_height,
            depth_or_array_layers: 1,
        };

        let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("diffuse texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &img_data,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * img_width),
                rows_per_image: Some(img_height),
            },
            texture_size,
        );

        let diffuse_texture_view =
            diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Scene shader"),
            source: wgpu::ShaderSource::Wgsl(
                r#"
            struct Uniforms {
                transform: mat4x4<f32>,
            };
            @group(0) @binding(0) var<uniform> uniforms: Uniforms;
            @group(0) @binding(1) var t_diffuse: texture_2d<f32>;
            @group(0) @binding(2) var s_diffuse: sampler;

            struct VertexInput {
                @location(0) position: vec3<f32>,
                @location(1) tex_coords: vec2<f32>,
            };

            struct VertexOutput {
                @builtin(position) clip_position: vec4<f32>,
                @location(0) tex_coords: vec2<f32>,
            };

            @vertex
            fn vs_main(model: VertexInput) -> VertexOutput {
                var out: VertexOutput;
                out.tex_coords = model.tex_coords;
                out.clip_position = uniforms.transform * vec4<f32>(model.position, 1.0);
                return out;
            }

            @fragment
            fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
                return textureSample(t_diffuse, s_diffuse, in.tex_coords);
            }
            "#
                .into(),
            ),
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = VERTICES.len() as u32;

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render pipeline layout"),
                bind_group_layouts: &[Some(&bind_group_layout)],
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
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

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
            input: InputState::default(),
            audio_manager,
            current_state: AppState::MainMenu,
            menu_scene: MainMenuState::new(),
            game_scene: GamePlayState::new(),
            diffuse_texture_view,
            diffuse_sampler,
            bind_group_layout,
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

    pub fn set_state(&mut self, next: AppState) {
        if next == AppState::Playing {
            println!("Entering Gameplay State.");
            self.game_scene = GamePlayState::new();
        }
        self.current_state = next;
    }

    pub fn update(&mut self, dt: f32) {
        let next_state = match self.current_state {
            AppState::MainMenu => self.menu_scene.update(dt, &self.input),
            AppState::Playing => self.game_scene.update(dt, &self.input),
            AppState::GameOver => None,
        };

        if let Some(new_state) = next_state {
            self.set_state(new_state);
        }
    }

    pub fn handle_keyboard(&mut self, event: &KeyEvent) -> Option<AppState> {
        match self.current_state {
            AppState::MainMenu => self.menu_scene.handle_keyboard(event),
            AppState::Playing => self.game_scene.handle_keyboard(event),
            AppState::GameOver => None,
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

                if self.current_state == AppState::Playing {
                    render_pass.set_pipeline(&self.render_pipeline);
                    render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                    let (width, height) = self.size.into();
                    let proj_matrix = self
                        .game_scene
                        .camera
                        .build_view_projection_matrix(width, height);

                    for (_, transdorm) in &mut self.game_scene.world.query::<(Entity, &Transform)>()
                    {
                        let mvp = proj_matrix * transdorm.to_matrix();
                        let uniform_data = Uniforms {
                            mvp: mvp.to_cols_array_2d(),
                        };

                        let uniform_buffer =
                            self.device
                                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                    label: Some("Temp uniform"),
                                    contents: bytemuck::cast_slice(&[uniform_data]),
                                    usage: wgpu::BufferUsages::UNIFORM,
                                });
                        let bind_group =
                            self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                                layout: &self.bind_group_layout,
                                entries: &[
                                    wgpu::BindGroupEntry {
                                        binding: 0,
                                        resource: uniform_buffer.as_entire_binding(),
                                    },
                                    wgpu::BindGroupEntry {
                                        binding: 1,
                                        resource: wgpu::BindingResource::TextureView(
                                            &self.diffuse_texture_view,
                                        ),
                                    },
                                    wgpu::BindGroupEntry {
                                        binding: 2,
                                        resource: wgpu::BindingResource::Sampler(
                                            &self.diffuse_sampler,
                                        ),
                                    },
                                ],
                                label: Some("temp bind group"),
                            });

                        render_pass.set_bind_group(0, &bind_group, &[]);
                        render_pass.draw(0..self.num_vertices, 0..1);
                    }
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
