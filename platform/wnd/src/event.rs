pub struct WindowId {}

#[derive(Clone, Debug)]
pub struct MouseInput {
    pub pos_x: i32,
    pub pos_y: i32,
}

#[derive(Clone, Debug)]
pub enum WindowEvent {
    MouseDown(MouseInput),
    MouseUp(MouseInput),
}

#[derive(Clone, Debug)]
pub enum Event {
    Init,
    WindowEvent(WindowEvent),
    Exit(ExitCode),
    None,
}

#[derive(Clone, Debug)]
pub enum ExitCode {
    Success,
}

#[derive(Clone, Debug)]
pub enum RunMode {
    Poll,
    Wait,
}
