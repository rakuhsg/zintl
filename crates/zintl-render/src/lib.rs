use std::{sync::Arc, time::Instant};

use wgpu::WgpuApplication;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};
use zintl::app::App;

mod wgpu;

pub struct Application<'a> {
    wgpu: Option<WgpuApplication<'a>>,
    window: Option<Arc<Window>>,
}

impl<'a> Application<'a> {
    pub fn new() -> Self {
        Self {
            wgpu: None,
            window: None,
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
                    wgpu.render();
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

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = Application::new();
    event_loop.run_app(&mut app);
}
