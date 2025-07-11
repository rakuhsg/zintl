use std::sync::{Arc, Mutex};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use zintl_render::{tessellator, text};
use zintl_render_math::{Alignment, LogicalPixelsPoint, LogicalPixelsRect, ScaleFactor, Viewport};
use zintl_ui::{App, RenderContent, RenderNode, RenderObject};
use zintl_wgpu::WgpuApplication;

#[allow(unused)]
#[derive(Debug)]
pub struct Application<'a> {
    app: App,
    wgpu: Option<Arc<Mutex<WgpuApplication<'a>>>>,
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
            app,
            wgpu: None,
            window: None,
            viewport: Viewport::new(800.into(), 600.into(), scale_factor),
            render_contents: vec![],
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

fn recursively_get_render_objects(node: &RenderNode, objects: &mut Vec<RenderObject>) {
    objects.push(node.object.clone());
    if let Some(node) = &node.inner {
        recursively_get_render_objects(node.as_ref(), objects);
    }
    for child in &node.children {
        recursively_get_render_objects(&child, objects);
    }
}

impl<'a> Application<'a> {
    pub fn get_render_objects(&mut self) -> Vec<RenderObject> {
        let mut objects = vec![];
        let node = self.app.root();
        recursively_get_render_objects(&node, &mut objects);
        objects
    }
    pub fn render(&mut self, event_loop: &ActiveEventLoop) {
        let wgpu = match self.wgpu.clone() {
            Some(wgpu) => wgpu,
            None => {
                event_loop.set_control_flow(ControlFlow::Poll);
                return;
            }
        };
        let mut wgpu = wgpu.lock().unwrap();
        let mut tessellation_jobs = Vec::new();

        let system_font = self.system_font.clone();

        let font = self
            .typecase
            .get_font(system_font)
            .expect("Failed to get system font family")
            .clone();

        let objs = self.get_render_objects();

        for obj in objs {
            match obj.content {
                RenderContent::Text(text) => {
                    let atlas_size = font.get_atlas_size();
                    let pixels = font.get_atlas_pixels();
                    let _ = wgpu.register_texture_with_id(0, pixels, atlas_size);

                    let galley = self.typesetter.compose(
                        text.as_str(),
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
            }
        }

        let mut meshes = Vec::with_capacity(2048);
        for job in tessellation_jobs {
            let m = self.tessellator.tessellate(&job, &self.viewport);
            meshes.extend(m);
        }

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
            Ok(wgpu) => Some(Arc::new(wgpu.into())),
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
                    wgpu.lock().unwrap().resize(self.viewport);
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
