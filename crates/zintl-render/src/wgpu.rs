use std::{borrow::Cow, fmt::Display, sync::Arc};

use wgpu::util::DeviceExt;
use wgpu::*;
use winit::window::Window;

use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::family_name::FamilyName;
use font_kit::hinting::HintingOptions;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::{Vector2F, Vector2I};

const SRC: &str = r###"

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
    out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
    return out;
}

/*@vertex
fn vs_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position, 0.0, 1.0);
}*/

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
"###;

struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

pub struct WgpuApplication<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    width: u32,
    height: u32,
    render_pipeline: wgpu::RenderPipeline,
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
                tex_coords: [0.0, 1.0],
            });
            vertices.push(Vertex {
                position: [x + width, y],
                tex_coords: [1.0, 1.0],
            });
            vertices.push(Vertex {
                position: [x + width, y + height],
                tex_coords: [1.0, 0.0],
            });

            // Bottom-left, top-right, top-left
            vertices.push(Vertex {
                position: [x, y],
                tex_coords: [0.0, 1.0],
            });
            vertices.push(Vertex {
                position: [x, y + height],
                tex_coords: [0.0, 0.0],
            });
            vertices.push(Vertex {
                position: [x + width, y + height],
                tex_coords: [1.0, 0.0],
            });
        }
        vertices
    }

    fn create_vertex_buffer(device: wgpu::Device, vertices: &[Vertex]) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: unsafe {
                std::slice::from_raw_parts(
                    vertices.as_ptr() as *const u8,
                    vertices.len() * std::mem::size_of::<Vertex>(),
                )
            },
            usage: wgpu::BufferUsages::VERTEX,
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

        // Font
        let font = SystemSource::new()
            .select_by_postscript_name("Hiragino Kaku Gothic ProN W3")
            .unwrap()
            .load()
            .unwrap();
        let mut canvas = Canvas::new(Vector2I::splat(256), Format::A8);
        let glyph_id = font.glyph_for_char('盆').unwrap();
        font.rasterize_glyph(
            &mut canvas,
            glyph_id,
            128.0,
            Transform2F::from_translation(Vector2F::new(0.0, 128.0)),
            HintingOptions::None,
            RasterizationOptions::GrayscaleAa,
        )
        .unwrap();
        let glyph_id = font.glyph_for_char('地').unwrap();
        font.rasterize_glyph(
            &mut canvas,
            glyph_id,
            128.0,
            Transform2F::from_translation(Vector2F::new(128.0, 128.0)),
            HintingOptions::None,
            RasterizationOptions::GrayscaleAa,
        )
        .unwrap();

        let texture_size = wgpu::Extent3d {
            width: canvas.size.x() as u32,
            height: canvas.size.y() as u32,
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            depth_or_array_layers: 1,
        };
        let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1, // We'll talk about this a little later
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
        let mut rgba_pixels = vec![0u8; (canvas.size.x() * canvas.size.y() * 4) as usize];
        for (i, &alpha) in canvas.pixels.iter().enumerate() {
            let base = i * 4;
            rgba_pixels[base] = 255 - alpha; // R
            rgba_pixels[base + 1] = 255 - alpha; // G
            rgba_pixels[base + 2] = 255 - alpha; // B
            rgba_pixels[base + 3] = alpha; // A
        }

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
                bytes_per_row: Some(4 * canvas.size.x() as u32),
                rows_per_image: Some(canvas.size.y() as u32),
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
            bind_group_layouts: &[&texture_bind_group_layout],
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
                targets: &[Some(swapchain_format.into())],
            }),
            multiview: None,
            cache: None,
        });

        let config = surface.get_default_config(&adapter, width, height).unwrap();
        surface.configure(&device, &config);

        let frame = surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        //
        let vertices = Self::rectangles_to_vertices(vec![
            [-0.25, -0.25, 0.5, 0.5],
            [-1., 0., 0.1, 0.1],
            [0.5, 0.5, 0.5, 0.5],
            [-0.5, -0.5, 0.5, 0.5],
        ]);
        let vertex_buffer = Self::create_vertex_buffer(device.clone(), &vertices);
        //

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.set_pipeline(&render_pipeline);
            rpass.set_bind_group(0, &diffuse_bind_group, &[]);
            rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
            rpass.draw(0..vertices.len() as u32, 0..1);
        }

        queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(WgpuApplication {
            surface,
            device,
            queue,
            config,
            width,
            height,
            render_pipeline,
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

    fn reconfigure_surface_size(&mut self) {
        self.config.width = self.width;
        self.config.height = self.height;
        self.surface.configure(&self.device, &self.config);
    }
}
