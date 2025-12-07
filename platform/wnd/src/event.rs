use std::sync::Arc;
use crate::{
    driver::EventRunner,
    window::{Window, WindowResult},
};
use crate::window::WindowInitialInfo;

/// Provides methods to create and modify windows.
pub struct Context {
    pub(crate) runner: Arc<EventRunner>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            runner: Arc::new(EventRunner::new()),
        }
    }

    pub fn create_window(&self, info: WindowInitialInfo) -> WindowResult<Window> {
        Window::new(self.runner.clone(), info)
    }
}

pub enum Event {}

pub enum ReturnCode {
    Exit,
}

/// Dispatches events and ships them to related handlers.
///
/// # Create an event loop
///
/// First, you need to create [`EvnetDispatcher`] instance.
///
/// ```no_run
/// use wnd::event::{Context, Event, EventDispatcher, EventHandler, ReturnCode};
/// use wnd::window::Window;
///
/// struct App {}
///
/// impl EventHandler for App {
///     fn init(&mut self, context: &Context) {
///         todo!()
///     }
///
///     fn window_event(&mut self, context: &Context, window: &Window, event: Event) {
///         todo!()
///     }
/// }
///
/// let dispatcher = EventDispatcher::new();
///
/// dispatcher.with_handler(App {});
///
/// loop {
///     match dispatcher.dispatch() {
///
///         Some(code) => match code {
///             ReturnCode::Exit => break,
///         },
///         _ => {}
///     }
/// }
/// ```
///
/// Next, you must register an application instance that implements [`EventHandler`] trait.
///
/// To run window, normally, you need to call [`dispatch`]
/// on an application main loop to dispatching window events.
///
/// Note: Currently `EventDispatcher` can be used on the same thread.
pub struct EventDispatcher {
    context: Context,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            context: Context::new(),
        }
    }

    pub fn with_handler<T: EventHandler>(&self, mut handler: T) {
        handler.init(&self.context);
        self.context.runner.register_handler(move |e| match e {})
    }

    pub fn dispatch(&self) -> Option<ReturnCode> {
        self.context.runner.dispatch_events()
    }
}

/// Functions are must be running on the same thread.
pub trait EventHandler {
    fn init(&mut self, context: &Context);
    fn window_event(&mut self, context: &Context, window: &Window, event: Event);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn event_loop() {
        let dispatcher = EventDispatcher::new();

        #[derive(Default)]
        struct App {
            window: Option<Window>,
        }

        impl EventHandler for App {
            fn init(&mut self, context: &Context) {
                let info = WindowInitialInfo {
                    pos_x: 0,
                    pos_y: 0,
                    width: 640,
                    height: 480,
                    title: String::from("wnd test"),
                };
                let window = context.create_window(info).expect("unable to create window");
                window.apply_system_appearance();
                self.window = Some(window);
            }
            fn window_event(&mut self, context: &Context, window: &Window, event: Event) {}
        }

        dispatcher.with_handler(App::default());

        loop {
            match dispatcher.dispatch() {
                Some(code) => match code {
                    ReturnCode::Exit => break,
                },
                _ => {}
            }
        }
    }
}
