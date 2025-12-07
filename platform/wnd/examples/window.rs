use wnd::event::{Context, Event, EventDispatcher, EventHandler, ReturnCode};
use wnd::window::{Window, WindowInitialInfo};
fn main() {
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
                title: String::from("window"),
            };
            let window = context
                .create_window(info)
                .expect("unable to create window");
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
