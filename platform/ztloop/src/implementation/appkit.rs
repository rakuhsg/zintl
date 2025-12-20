use crate::{EventPump, implementation::appkit_binding as binding};

pub struct AppkitEventPump {}

impl AppkitEventPump {
    pub fn new() -> Self {
        unsafe { binding::ztloop_init() };
        AppkitEventPump {}
    }
}

impl EventPump for AppkitEventPump {
    fn run(&mut self) {
        unsafe { binding::ztloop_run() };
    }

    fn quit(&mut self) {}
}
