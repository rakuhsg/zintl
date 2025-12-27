use crate::{EventPump, implementation::appkit_binding as binding};

pub struct AppkitEventPump {}

impl AppkitEventPump {
    pub fn new() -> Self {
        unsafe { binding::ztloopa_init() };
        AppkitEventPump {}
    }
}

impl EventPump for AppkitEventPump {
    fn run(&mut self) {
        unsafe { binding::ztloopa_run() };
    }

    fn quit(&mut self) {
        unsafe { binding::ztloopa_destroy() };
    }
}
