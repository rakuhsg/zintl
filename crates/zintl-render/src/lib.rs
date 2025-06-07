use std::sync::Arc;

use wgpu::WgpuApplication;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};
use zintl::{app::App, render::RenderContent};

pub mod layout;
pub mod mesh;
mod render_object;
pub mod scaling;
mod tessellator;
pub mod text;
mod texture;
pub mod wgpu;

use scaling::LogicalPoint;

#[allow(unused)]
#[derive(Debug)]
pub struct Application<'a> {
    root: App,
    wgpu: Option<WgpuApplication<'a>>,
    window: Option<Arc<Window>>,
    viewport: scaling::Viewport,
    render_contents: Vec<RenderContent>,
    tessellator: tessellator::Tessellator,
    system_font: text::FontProperties,
    typecase: text::Typecase,
    typesetter: text::Typesetter,
}

impl<'a> Application<'a> {
    pub fn new(app: App) -> Self {
        let mut typecase = text::Typecase::new();
        let fam = include_bytes!("../../../assets/inter/Inter-Regular.ttf").to_vec();
        typecase.load_font("Inter".to_string(), fam);
        Self {
            root: app,
            wgpu: None,
            window: None,
            viewport: scaling::Viewport {
                device_width: 800,
                device_height: 600,
                scale_factor: scaling::ScaleFactor {
                    device_pixel_ratio: 1.0,
                },
            },
            render_contents: vec![RenderContent::Text("Zintl".to_string())],
            tessellator: tessellator::Tessellator::new(),
            system_font: text::FontProperties {
                name: "Inter".to_string(),
                scale_string: "32.0".to_string(),
            },
            typecase,
            typesetter: text::Typesetter::new(),
        }
    }
}

impl<'a> Application<'a> {
    pub fn render(&mut self) {
        let wgpu = match &mut self.wgpu {
            Some(wgpu) => wgpu,
            None => return,
        };
        let mut tessellation_jobs = Vec::new();

        let family = self
            .typecase
            .get_font(self.system_font.clone())
            .expect("Failed to get system font family");

        self.render_contents
            .iter()
            .for_each(|content| match content {
                RenderContent::Text(text) => {
                    let atlas_size = family.get_atlas_size();
                    let pixels = family.get_atlas_pixels();
                    let _ = wgpu.register_texture_with_id(0, pixels, atlas_size);

                    // TODO
                    #[allow(unused)]
                    let galley =
                        self.typesetter
                            .compose(text, &family, LogicalPoint::new(0.0, 120.0));
                    tessellation_jobs.push(tessellator::TessellationJob::Galley(galley));
                }
                _ => {}
            });

        let meshes = self
            .tessellator
            .tessellate(&tessellation_jobs[0], &self.viewport);

        wgpu.draw(meshes);
    }
}

impl<'a> ApplicationHandler for Application<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_inner_size(
                    winit::dpi::LogicalSize {
                        width: self.viewport.device_width,
                        height: self.viewport.device_height,
                    },
                ))
                .unwrap(),
        );
        event_loop.set_control_flow(ControlFlow::Wait);
        self.window = Some(window.clone());
        self.wgpu =
            match pollster::block_on(WgpuApplication::from_window(window.clone(), self.viewport)) {
                Ok(wgpu) => Some(wgpu),
                Err(err) => {
                    eprintln!("Failed to create WGPU application: {}", err);
                    None
                }
            };
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render();
                event_loop.set_control_flow(ControlFlow::Wait);
            }
            WindowEvent::Resized(size) => {
                self.viewport.device_width = size.width as u32;
                self.viewport.device_height = size.height as u32;

                if let Some(wgpu) = &mut self.wgpu {
                    wgpu.resize(self.viewport);
                }
            }
            _ => (),
        }
    }
}

pub fn run_app(app: App) {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut w_app = Application::new(app);
    let _ = event_loop.run_app(&mut w_app);
}
