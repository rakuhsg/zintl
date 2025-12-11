pub struct WindowId {}

pub enum WindowEvent {}

pub enum Event {
    Init,
    WindowEvent(WindowEvent),
    Exit(ExitCode),
    None,
}

pub enum ExitCode {
    Success,
}

pub enum RunMode {
    Poll,
    Wait,
}
