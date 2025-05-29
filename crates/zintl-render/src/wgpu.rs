use std::{borrow::Cow, fmt::Display, sync::Arc};

use wgpu::util::DeviceExt;
use winit::window::Window;

use zintl_render_math::Mat4;

use std::collections::HashMap;

use crate::mesh::{Mesh, Uniforms, Vertex};

#[derive(Debug, Clone)]
pub struct Texture {
    #[allow(dead_code)]
    native_texture: wgpu::Texture,
    bind_group: wgpu::BindGroup,
}

const SRC: &str = r###"

struct Uniforms {
    ortho : mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms : Uniforms;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = uniforms.ortho * vec4<f32>(model.position, 0.0, 1.0);
    return out;
}

/*@vertex
fn vs_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position, 0.0, 1.0);
}*/

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
"###;

pub type TextureId = usize;

/// A instance of a WGPU renderer
#[derive(Debug)]
pub struct WgpuApplication<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    /// Chached surface width
    width: u32,
    /// Chached surface height
    height: u32,
    render_pipeline: wgpu::RenderPipeline,
    textures: HashMap<TextureId, Texture>,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    texture_bind_group_layout: wgpu::BindGroupLayout,
}

/// Error type for WgpuApplication
#[derive(Clone, Debug)]
pub enum WgpuApplicationError {
    CreateSurfaceError,
    AdapterRequestDeviceError(wgpu::RequestAdapterError),
    CreateDeviceError,
}

impl Display for WgpuApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WgpuApplicationError::CreateSurfaceError => {
                write!(f, "Failed to create surface")
            }
            WgpuApplicationError::AdapterRequestDeviceError(err) => {
                write!(f, "Failed to find an appropriate adapter: {:?}", err)
            }
            WgpuApplicationError::CreateDeviceError => {
                write!(f, "Failed to create device")
            }
        }
    }
}

/// Result type for WgpuApplication
pub type WgpuApplicationResult<T> = Result<T, WgpuApplicationError>;

impl<'a> WgpuApplication<'a> {
    async fn init_adapter(
        instance: &wgpu::Instance,
        surface: &wgpu::Surface<'a>,
    ) -> WgpuApplicationResult<wgpu::Adapter> {
        match instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(surface),
            })
            .await
        {
            Ok(a) => Ok(a),
            Err(err) => Err(WgpuApplicationError::AdapterRequestDeviceError(err)),
        }
    }

    /*fn rectangles_to_vertices(rectangles: Vec<[f32; 4]>) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        for [x, y, width, height] in rectangles {
            // Bottom-left, bottom-right, top-right
            vertices.push(Vertex {
                position: [x, y],
                tex_coords: [0.0, 0.0],
            });
            vertices.push(Vertex {
                position: [x + width, y],
                tex_coords: [1.0, 0.0],
            });
            vertices.push(Vertex {
                position: [x + width, y + height],
                tex_coords: [1.0, 1.0],
            });

            // Bottom-left, top-right, top-left
            vertices.push(Vertex {
                position: [x, y],
                tex_coords: [0.0, 0.0],
            });
            vertices.push(Vertex {
                position: [x, y + height],
                tex_coords: [0.0, 1.0],
            });
            vertices.push(Vertex {
                position: [x + width, y + height],
                tex_coords: [1.0, 1.0],
            });
        }
        vertices
    }*/

    fn create_ortho_matrix_from_size(width: f32, height: f32) -> Mat4 {
        cgmath::ortho(0., width, height, 0., -1., 1.).into()
    }

    fn create_vertex_buffer(device: &wgpu::Device, vertices: &[Vertex]) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    fn create_index_buffer(device: &wgpu::Device, indices: &[u32]) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        })
    }

    fn create_uniform_buffer(device: &wgpu::Device, uniforms: &Uniforms) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[*uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    async fn init(
        instance: wgpu::Instance,
        surface: wgpu::Surface<'a>,
        width: u32,
        height: u32,
    ) -> WgpuApplicationResult<Self> {
        let adapter = Self::init_adapter(&instance, &surface).await?;

        let (device, queue) = match adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
                trace: wgpu::Trace::Off,
            })
            .await
        {
            Ok(r) => r,
            Err(..) => return Err(WgpuApplicationError::CreateDeviceError),
        };

        // Orthographic projection matrix
        let ortho_matrix = Self::create_ortho_matrix_from_size(width as f32, height as f32);
        let uniform_buffer = Self::create_uniform_buffer(
            &device,
            &Uniforms {
                ortho: ortho_matrix.into(),
            },
        );
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
                label: Some("uniform_bind_group_layout"),
            });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(SRC)),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&uniform_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        // position [x, y]
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 0, // Matches @location(0) in the vertex shader
                        },
                        // tex_coords [x, y]
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 1,
                        },
                    ],
                }],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: swapchain_format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::Src,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        });

        let config = surface.get_default_config(&adapter, width, height).unwrap();
        surface.configure(&device, &config);

        Ok(WgpuApplication {
            surface,
            device,
            queue,
            config,
            width,
            height,
            render_pipeline,
            uniform_buffer,
            uniform_bind_group,
            textures: HashMap::new(),
            texture_bind_group_layout,
        })
    }

    pub async fn from_window(window: Arc<Window>) -> WgpuApplicationResult<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface_target = wgpu::SurfaceTarget::Window(Box::new(window.clone()));
        let surface = match instance.create_surface(surface_target) {
            Ok(s) => s,
            Err(..) => return Err(WgpuApplicationError::CreateSurfaceError),
        };

        let size = window.inner_size();
        let width = size.width;
        let height = size.height;

        Self::init(instance, surface, width, height).await
    }

    // TODO
    fn draw_mesh(&mut self, mesh: Mesh, rpass: &mut wgpu::RenderPass<'_>) {
        if let Some(texture_id) = mesh.texture_id {
            let texture = self.textures.get(&texture_id).expect("Texture not found");

            rpass.set_bind_group(1, &texture.bind_group, &[]);
        }
        let vertex_buffer = Self::create_vertex_buffer(&self.device, &mesh.vertices);
        let index_buffer = Self::create_index_buffer(&self.device, &mesh.indices);
        rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
        rpass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        rpass.draw_indexed(0..mesh.indices.len() as u32, 0, 0..1);

        if !mesh.children.is_empty() {
            for child in mesh.children {
                self.draw_mesh(child, rpass);
            }
        }
    }

    pub fn draw(&mut self, meshes: Vec<Mesh>) {
        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, &self.uniform_bind_group, &[]);

            for mesh in meshes {
                self.draw_mesh(mesh, &mut rpass);
            }
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
        /*rpass.set_bind_group(1, &self.diffuse_bind_group, &[]);
        rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
        rpass.draw(0..self.vertices.len() as u32, 0..1);*/
    }

    /*pub fn render(&mut self) {
        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.draw_objects(&mut rpass);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }*/

    /*pub async fn from_canvas(canvas: CanvasElement) -> WgpuDriverResult<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface_target = wgpu::SurfaceTarget::Canvas(canvas.elm);
        let surface = match instance.create_surface(surface_target) {
            Ok(s) => s,
            Err(..) => return Err(WgpuDriverError::CreateSurfaceError),
        };

        Self::init(instance, surface, canvas.width, canvas.height).await
    }*/

    pub fn resize(&mut self, width: u32, height: u32) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.reconfigure_surface_size();
        }
    }

    fn reconfigure_surface_size(&mut self) {
        self.config.width = self.width;
        self.config.height = self.height;
        let ortho = Self::create_ortho_matrix_from_size(self.width as f32, self.height as f32);
        let uniforms = Uniforms {
            ortho: ortho.into(),
        };
        self.queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
        self.surface.configure(&self.device, &self.config);
    }

    pub fn register_texture(&mut self, pixels: Vec<u8>, width: u32, height: u32) -> usize {
        let id = self.textures.len();
        self.register_texture_with_id(id, pixels, width, height)
    }

    pub fn register_texture_with_id(
        &mut self,
        id: usize,
        pixels: Vec<u8>,
        width: u32,
        height: u32,
    ) -> usize {
        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some(format!("texture_{}", id).as_str()),
            // TODO
            view_formats: &[],
        });

        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            texture_size,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // TODO: Cache sampler
        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some(format!("bind_group_{}", id).as_str()),
        });

        let texture = Texture {
            native_texture: texture,
            bind_group,
        };

        self.textures.insert(id, texture);
        id
    }

    // TODO: fn patch_texture
}
