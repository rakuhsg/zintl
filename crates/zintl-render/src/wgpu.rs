use std::{borrow::Cow, fmt::Display, sync::Arc};

use ab_glyph::{Font, Glyph, point};
use cgmath::{self, Matrix4};
use font_kit::handle::Handle;
use wgpu::util::DeviceExt;
use wgpu::*;
use winit::window::Window;

use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::hinting::HintingOptions;
use font_kit::source::SystemSource;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::{Vector2F, Vector2I};
use zintl::render::RenderObject;

use std::collections::HashMap;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub ortho: [[f32; 4]; 4],
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub texture_id: usize,
}

pub struct Texture {}

pub struct Renderer {
    textures: HashMap<usize, Texture>,
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
    vertices: Vec<Vertex>,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    ortho_matrix: Matrix4<f32>,
    diffuse_bind_group: wgpu::BindGroup,
}

/// Error type for WgpuApplication
#[derive(Debug)]
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

    fn rectangles_to_vertices(rectangles: Vec<[f32; 4]>) -> Vec<Vertex> {
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
    }

    fn create_ortho_matrix_from_size(width: f32, height: f32) -> Matrix4<f32> {
        cgmath::ortho(0., width, height, 0., -1., 1.)
    }

    fn create_vertex_buffer(device: &wgpu::Device, vertices: &[Vertex]) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
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
                trace: Trace::Off,
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

        // Font
        let font = SystemSource::new()
            .select_by_postscript_name("Hiragino Kaku Gothic ProN W3")
            .unwrap()
            .load()
            .unwrap();
        let font_size = [128, 128];

        let mut rgba_pixels = vec![0u8; (font_size[0] * 4 * font_size[1]) as usize];

        match font.handle().unwrap() {
            Handle::Path { path, font_index } => {
                println!("Path: {:?}", path.to_str());
                println!("Font index: {:?}", font_index);
            }
            Handle::Memory { bytes, font_index } => {
                let font = ab_glyph::FontVec::try_from_vec(bytes.to_vec()).unwrap();
                let q_glyph: Glyph = font
                    .glyph_id('善')
                    .with_scale_and_position(font_size[0] as f32, point(0.0, 0.0));

                // Draw it.
                if let Some(q) = font.outline_glyph(q_glyph) {
                    q.draw(|x, y, c| {
                        let base = (y * 4 * font_size[0] + x * 4) as usize;
                        rgba_pixels[base] = ((1. - c) * 255.) as u8; // R
                        rgba_pixels[base + 1] = ((1. - c) * 255.) as u8; // G
                        rgba_pixels[base + 2] = ((1. - c) * 255.) as u8; // B
                        rgba_pixels[base + 3] = (c * 255.) as u8; // A
                    });
                }
            }
        }

        /*let metrics = font.metrics();
        let ascent = metrics.ascent as f32 / metrics.units_per_em as f32 * 128.0;
        let descent = metrics.descent as f32 / metrics.units_per_em as f32 * 128.0;

        let mut canvas = Canvas::new(Vector2I::new(256, 150), Format::A8);
        let glyph_id = font.glyph_for_char('こ').unwrap();

        font.rasterize_glyph(
            &mut canvas,
            glyph_id,
            128.0,
            Transform2F::from_translation(Vector2F::new(0., ascent)),
            HintingOptions::None,
            RasterizationOptions::GrayscaleAa,
        )
        .unwrap();
        let glyph_id = font.glyph_for_char('ん').unwrap();
        font.rasterize_glyph(
            &mut canvas,
            glyph_id,
            128.0,
            Transform2F::from_translation(Vector2F::new(128.0 + descent, ascent)),
            HintingOptions::None,
            RasterizationOptions::GrayscaleAa,
        )
        .unwrap();*/

        let texture_size = wgpu::Extent3d {
            width: font_size[0] as u32,
            height: font_size[1] as u32,
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            depth_or_array_layers: 1,
        };
        let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Most images are stored using sRGB, so we need to reflect that here.
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
            // COPY_DST means that we want to copy data to this texture
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("diffuse_texture"),
            // This is the same as with the SurfaceConfig. It
            // specifies what texture formats can be used to
            // create TextureViews for this texture. The base
            // texture format (Rgba8UnormSrgb in this case) is
            // always supported. Note that using a different
            // texture format is not supported on the WebGL2
            // backend.
            view_formats: &[],
        });
        /* let mut rgba_pixels = vec![0u8; (font_size[0] * font_size[1] * 4) as usize];
        for (i, &alpha) in canvas.pixels.iter().enumerate() {
            let base = i * 4;
            rgba_pixels[base] = 255 - alpha; // R
            rgba_pixels[base + 1] = 255 - alpha; // G
            rgba_pixels[base + 2] = 255 - alpha; // B
            rgba_pixels[base + 3] = alpha; // A
        }*/

        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::TexelCopyTextureInfo {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            &rgba_pixels,
            // The layout of the texture
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * font_size[0] as u32),
                rows_per_image: Some(font_size[1] as u32),
            },
            texture_size,
        );
        let diffuse_texture_view =
            diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
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
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
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

        let vertices =
            Self::rectangles_to_vertices(vec![[0., 0., font_size[0] as f32, font_size[1] as f32]]);

        Ok(WgpuApplication {
            surface,
            device,
            queue,
            config,
            width,
            height,
            render_pipeline,
            vertices,
            uniform_buffer,
            uniform_bind_group,
            ortho_matrix,
            diffuse_bind_group,
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

    fn draw_objects(&mut self, rpass: &mut wgpu::RenderPass<'_>) {
        let vertex_buffer = Self::create_vertex_buffer(&self.device, &self.vertices);
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_bind_group(0, &self.uniform_bind_group, &[]);
        rpass.set_bind_group(1, &self.diffuse_bind_group, &[]);
        rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
        rpass.draw(0..self.vertices.len() as u32, 0..1);
    }

    pub fn render(&mut self, render_objects: Arc<Vec<RenderObject>>) {
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
    }

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
        let ortho = cgmath::ortho(0.0, self.width as f32, self.height as f32, 0.0, -1.0, 1.0);
        let uniforms = Uniforms {
            ortho: ortho.into(),
        };
        self.queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
        self.surface.configure(&self.device, &self.config);
    }
}
