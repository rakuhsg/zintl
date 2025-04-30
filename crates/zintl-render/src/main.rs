use std::sync::Arc;
use wgpu::{include_wgsl, util::DeviceExt};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};
use zintl::app::{App, ComposableView, Context, Label, Stack, View};

struct ZintlRenderer<V: View> {
    app: App,
    root: Option<V>,
    context: Context,
}

impl<V: View> ZintlRenderer<V> {
    fn new(app: App) -> Self {
        Self {
            app,
            root: None,
            context: Context::new(),
        }
    }

    fn set_root(&mut self, root: V) {
        self.root = Some(root);
    }

    fn render_view(&self, view: &V) {
        // For now just log that we're rendering
        println!("Rendering view");
    }

    fn run(mut self, root: V) {
        let event_loop = EventLoop::new().unwrap();
        let window = Arc::new(Window::new(&event_loop).unwrap());
        window.set_title("Zintl");

        self.set_root(root);
        let mut state = pollster::block_on(WgpuState::new(window.clone(), self));

        let _ = event_loop.run(move |event, elwt| match event {
            Event::WindowEvent { window_id, event } => {
                if window_id == window.id() {
                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::Resized(physical_size) => {
                            state.resize(physical_size);
                        }
                        WindowEvent::RedrawRequested => match state.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                            Err(e) => eprintln!("{:?}", e),
                        },
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        });
    }
}

struct WgpuState<V: View> {
    surface: wgpu::Surface<'static>,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    renderer: ZintlRenderer<V>,
}

impl<V: View> WgpuState<V> {
    async fn new(window: Arc<Window>, renderer: ZintlRenderer<V>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let device = Arc::new(device);
        let queue = Arc::new(queue);

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
        };
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            renderer,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.9,
                            g: 0.9,
                            b: 0.9,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Render the view hierarchy
            if let Some(root) = &self.renderer.root {
                self.renderer.render_view(root);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }
}

fn main() {
    let app = App::new();
    let renderer = ZintlRenderer::new(app);

    // Create a simple test UI with just a root stack
    let root = Stack::new().padding(10.0, 10.0, 10.0, 10.0);
    renderer.run(root);
}
