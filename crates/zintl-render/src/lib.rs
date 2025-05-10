use std::sync::Arc;

use wgpu::WgpuApplication;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};
use zintl::{app::App, render::RenderObject};

mod render_object;
mod wgpu;

pub struct Application<'a> {
    root: App,
    wgpu: Option<WgpuApplication<'a>>,
    window: Option<Arc<Window>>,
    render_objects: Arc<Vec<RenderObject>>,
}

impl<'a> Application<'a> {
    pub fn new(app: App) -> Self {
        Self {
            root: app,
            wgpu: None,
            window: None,
            render_objects: Arc::new(vec![]),
        }
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

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(wgpu) = &mut self.wgpu {
                    wgpu.render(self.render_objects.clone());
                }
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                //self.window.as_ref().unwrap().request_redraw();
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
