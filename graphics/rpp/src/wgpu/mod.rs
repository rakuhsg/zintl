use std::borrow::Cow;
use std::collections::HashMap;

const PP_SHADER_SRC: &str = include_str!("./shader.wgsl");

use crate::geometry::{Mat4, Size, Viewport};

#[cfg(feature = "rwh")]
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::util::DeviceExt;

pub trait WindowHandle: HasWindowHandle + HasDisplayHandle + Sync + Send {}

pub type TextureId = usize;

pub struct Texture {
    wgpu_texture: wgpu::Texture,
    bind_group: wgpu::BindGroup,
    size: Size,
}

#[derive(Clone, Debug)]
pub enum WgpuErr {
    AdapterRequestDeviceError(wgpu::RequestAdapterError),
    CreateSurfaceError,
    CreateDeviceError,
}

pub type WgpuResult<T> = Result<T, WgpuErr>;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub ortho: Mat4,
}

struct WgpuVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    color: f32,
}

pub struct RenderPipeline {
    rp: wgpu::RenderPipeline,
    textures: HashMap<TextureId, Texture>,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl RenderPipeline {
    fn create_uniform_buffer(device: &wgpu::Device, uniforms: &Uniforms) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[*uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    pub fn new(
        device: &wgpu::Device,
        viewport: &Viewport,
        color_format: wgpu::TextureFormat,
    ) -> Self {
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

        let ortho_matrix = viewport.to_ortho();
        let uniform_buffer = Self::create_uniform_buffer(
            &device,
            &Uniforms {
                ortho: ortho_matrix.into(),
            },
        );

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
                    // Texture binding
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
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(PP_SHADER_SRC)),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&uniform_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        // position, tex coords, color
        let attributes = &wgpu::vertex_attr_array![
            0 => Float32x2,
            1 => Float32x2,
            2 => Uint32
        ];

        let rp = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<WgpuVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes,
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
                    format: color_format,
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

        RenderPipeline {
            uniform_buffer,
            uniform_bind_group,
            texture_bind_group_layout,
            textures: HashMap::new(),
            rp,
        }
    }
}

pub struct Surface<'a> {
    surface: wgpu::Surface<'a>,
    config: wgpu::SurfaceConfiguration,
}

pub struct WgpuDevice<'a> {
    surface: Option<Surface<'a>>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: RenderPipeline,
}

impl<'a> WgpuDevice<'a> {
    fn create_instance() -> wgpu::Instance {
        wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        })
    }

    #[cfg(feature = "rwh")]
    fn create_surface_rwh_window<T: WindowHandle + 'a>(
        instance: &wgpu::Instance,
        window: T,
    ) -> WgpuResult<wgpu::Surface<'a>> {
        let surface_target = wgpu::SurfaceTarget::Window(Box::new(window));
        let surface = match instance.create_surface(surface_target) {
            Ok(s) => s,
            Err(..) => return Err(WgpuErr::CreateSurfaceError),
        };
        Ok(surface)
    }

    async fn init_adapter(
        instance: &wgpu::Instance,
        surface: Option<&wgpu::Surface<'a>>,
    ) -> WgpuResult<wgpu::Adapter> {
        match instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: surface,
            })
            .await
        {
            Ok(a) => Ok(a),
            Err(err) => Err(WgpuErr::AdapterRequestDeviceError(err)),
        }
    }

    async fn init(
        adapter: &wgpu::Adapter,
        wgpu_surface: Option<wgpu::Surface<'a>>,
        viewport: &Viewport,
        color_format: wgpu::TextureFormat,
    ) -> WgpuResult<Self> {
        let (device, queue) = match adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
                trace: wgpu::Trace::Off,
                ..Default::default()
            })
            .await
        {
            Ok(r) => r,
            Err(..) => return Err(WgpuErr::CreateDeviceError),
        };

        let pipeline = RenderPipeline::new(&device, viewport, color_format);

        let surface = match wgpu_surface {
            Some(wgpu_surface) => {
                let config = wgpu_surface
                    .get_default_config(&adapter, viewport.width as u32, viewport.height as u32)
                    .unwrap();
                wgpu_surface.configure(&device, &config);
                Some(Surface {
                    surface: wgpu_surface,
                    config: config,
                })
            }
            None => None,
        };

        Ok(WgpuDevice {
            device,
            surface,
            queue,
            pipeline,
        })
    }

    #[cfg(feature = "rwh")]
    pub async fn with_window<T: WindowHandle + 'a>(
        window: T,
        viewport: &Viewport,
        color_format: Option<wgpu::TextureFormat>,
    ) -> WgpuResult<Self> {
        let instance = Self::create_instance();
        let surface = Self::create_surface_rwh_window(&instance, window)?;
        let adapter = Self::init_adapter(&instance, Some(&surface)).await?;
        let color_format = color_format.unwrap_or_else(|| {
            let swapchain_capabilities = surface.get_capabilities(&adapter);
            swapchain_capabilities.formats[0]
        });

        Self::init(&adapter, Some(surface), viewport, color_format).await
    }

    pub async fn new(
        viewport: &Viewport,
        color_format: Option<wgpu::TextureFormat>,
    ) -> WgpuResult<Self> {
        let instance = Self::create_instance();
        let adapter = Self::init_adapter(&instance, None).await?;

        Self::init(
            &adapter,
            None,
            viewport,
            color_format.unwrap_or(wgpu::TextureFormat::Rgba8UnormSrgb),
        )
        .await
    }
}

pub async fn create_device<'a>(viewport: &Viewport) -> WgpuResult<WgpuDevice<'a>> {
    WgpuDevice::new(viewport, None).await
}
