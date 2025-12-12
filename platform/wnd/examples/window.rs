use wnd::prelude::*;

fn main() {
    let mut platform = Platform::new(RunMode::Poll).unwrap();
    #[allow(unused)]
    let mut window: Option<Window> = None;

    loop {
        match platform.dispatch() {
            Event::Init => {
                let info = WindowInitialInfo {
                    pos_x: 0,
                    pos_y: 0,
                    width: 640,
                    height: 480,
                    title: String::from("window"),
                };
                let win = platform
                    .create_window(info)
                    .expect("unable to create window");
                win.apply_system_appearance();
                window = Some(win);
            }
            Event::WindowEvent(we) => match we {
                WindowEvent::MouseDown(i) => {
                    if let Some(window) = &window {
                        window.set_title(format!("{}", i.pos_x));
                    }
                }
                _ => {}
            },
            Event::Exit(code) => match code {
                ExitCode::Success => {
                    //window.terminate_or(None);
                    break;
                }
            },
            _ => {}
        }
    }
}
