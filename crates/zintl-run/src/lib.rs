use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use zintl_render::{tessellator, text};
use zintl_render_math::{Alignment, LogicalPixelsPoint, LogicalPixelsRect, ScaleFactor, Viewport};
use zintl_ui::{App, RenderContent};
use zintl_wgpu::WgpuApplication;

#[allow(unused)]
#[derive(Debug)]
pub struct Application<'a> {
    root: App,
    wgpu: Option<WgpuApplication<'a>>,
    window: Option<Arc<Window>>,
    viewport: Viewport,
    render_contents: Vec<RenderContent>,
    tessellator: tessellator::Tessellator,
    system_font: text::FontProperties,
    typecase: text::Typecase,
    typesetter: text::Typesetter,
}

impl<'a> Application<'a> {
    pub fn new(app: App) -> Self {
        let scale_factor = ScaleFactor::new(96.0, 1.25);
        let mut typecase = text::Typecase::new(scale_factor.clone());
        let fam = include_bytes!("../assets/inter/Inter-Regular.ttf").to_vec();
        typecase.load_font("Inter".to_string(), fam);
        Self {
            root: app,
            wgpu: None,
            window: None,
            viewport: Viewport::new(800.into(), 600.into(), scale_factor),
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
    pub fn render(&mut self, event_loop: &ActiveEventLoop) {
        let wgpu = match &mut self.wgpu {
            Some(wgpu) => wgpu,
            None => {
                event_loop.set_control_flow(ControlFlow::Poll);
                return;
            }
        };
        let mut tessellation_jobs = Vec::new();

        let font = self
            .typecase
            .get_font(self.system_font.clone())
            .expect("Failed to get system font family");

        self.render_contents
            .iter()
            .for_each(|content| match content {
                RenderContent::Text(text) => {
                    let atlas_size = font.get_atlas_size();
                    let pixels = font.get_atlas_pixels();
                    let _ = wgpu.register_texture_with_id(0, pixels, atlas_size);

                    let galley = self.typesetter.compose(
                        text,
                        &font,
                        LogicalPixelsRect::new(
                            LogicalPixelsPoint::zero(),
                            LogicalPixelsPoint::new(800.0.into(), 600.0.into()),
                        ),
                        text::TextAlignment::Left,
                        Alignment::TopLeft,
                        &self.viewport.scale_factor,
                    );
                    tessellation_jobs.push(tessellator::TessellationJob::Galley(galley));
                }
                _ => {}
            });

        let meshes = self
            .tessellator
            .tessellate(&tessellation_jobs[0], &self.viewport);

        wgpu.draw(meshes);

        event_loop.set_control_flow(ControlFlow::Wait);
    }
}

impl<'a> ApplicationHandler for Application<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_inner_size(
                    winit::dpi::LogicalSize {
                        width: self.viewport.device_width.value(),
                        height: self.viewport.device_height.value(),
                    },
                ))
                .unwrap(),
        );
        self.window = Some(window.clone());
        println!("Window scale factor: {}", window.scale_factor());
        self.wgpu = match pollster::block_on(WgpuApplication::from_window_handle(
            window.clone(),
            self.viewport,
        )) {
            Ok(wgpu) => Some(wgpu),
            Err(err) => {
                eprintln!("Failed to create WGPU application: {}", err);
                None
            }
        };
        self.render(event_loop);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render(event_loop);
            }
            WindowEvent::Resized(size) => {
                self.viewport.device_width = (size.width as u32).into();
                self.viewport.device_height = (size.height as u32).into();

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

    let mut w_app = Application::new(app);
    let _ = event_loop.run_app(&mut w_app);
}
