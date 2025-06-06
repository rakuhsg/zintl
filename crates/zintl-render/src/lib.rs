use std::sync::Arc;

use wgpu::WgpuApplication;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};
use zintl::{app::App, render::RenderContent};
use zintl_render_math::{Point, Vec2};

pub mod mesh;
mod render;
mod render_object;
mod tessellator;
pub mod text;
mod texture;
pub mod wgpu;

#[allow(unused)]
#[derive(Debug)]
pub struct Application<'a> {
    root: App,
    wgpu: Option<WgpuApplication<'a>>,
    window: Option<Arc<Window>>,
    render_contents: Vec<RenderContent>,
    tessellator: tessellator::Tessellator,
    system_font: text::FamilyProperties,
    family_manager: text::FamilyManager,
    typesetter: text::Typesetter,
}

impl<'a> Application<'a> {
    pub fn new(app: App) -> Self {
        let mut family_manager = text::FamilyManager::new();
        let fam = include_bytes!("../../../assets/inter/Inter-Regular.ttf").to_vec();
        family_manager.load_family("Inter".to_string(), fam);
        Self {
            root: app,
            wgpu: None,
            window: None,
            render_contents: vec![RenderContent::Text("Fros".to_string())],
            tessellator: tessellator::Tessellator::new(),
            system_font: text::FamilyProperties {
                name: "Inter".to_string(),
                scale_string: "32.0".to_string(),
            },
            family_manager,
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
        let mut meshes = Vec::new();

        let family = self
            .family_manager
            .get_family(self.system_font.clone())
            .expect("Failed to get system font family");

        self.render_contents
            .iter()
            .for_each(|content| match content {
                RenderContent::Text(text) => {
                    let family = self
                        .family_manager
                        .get_family(self.system_font.clone())
                        .expect("Failed to get system font family");

                    let atlas_size = family.get_atlas_size();
                    let pixels = family.get_atlas_pixels();
                    let _ = wgpu.register_texture_with_id(
                        0,
                        pixels,
                        atlas_size.0 as u32,
                        atlas_size.1 as u32,
                    );

                    let galley = self
                        .typesetter
                        .compose(text, &family, Point::new(0.0, 120.0));
                    let inner_meshes = galley.to_meshes(atlas_size.into());
                    meshes.push(mesh::Mesh::from_children(inner_meshes));
                }
                _ => {}
            });

        wgpu.draw(meshes);
    }
}

impl<'a> ApplicationHandler for Application<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        event_loop.set_control_flow(ControlFlow::Wait);
        self.window = Some(window.clone());
        self.wgpu = match pollster::block_on(WgpuApplication::from_window(window.clone())) {
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
                if let Some(wgpu) = &mut self.wgpu {
                    wgpu.resize(size.width, size.height);
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
