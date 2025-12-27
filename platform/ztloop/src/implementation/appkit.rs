use crate::{implementation::appkit_binding as binding, osloop::OSLoop};

pub struct AppkitOSLoop {}

impl AppkitOSLoop {
    pub fn new() -> Self {
        unsafe { binding::ztloopa_init() };
        AppkitOSLoop {}
    }
}

impl OSLoop for AppkitOSLoop {
    fn run(&mut self) {
        unsafe { binding::ztloopa_run() };
    }

    fn quit(&mut self) {
        unsafe { binding::ztloopa_destroy() };
    }
}
