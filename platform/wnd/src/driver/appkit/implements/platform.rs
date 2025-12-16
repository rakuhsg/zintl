use std::sync::mpsc;

use super::window::WindowHandler;
use crate::{
    driver::{appkit::binding, types::*},
    event::Event,
    window::{Window, WindowInitialInfo},
};

pub struct PlatformImpl {}

impl PlatformImpl {
    pub fn new(_sender: mpsc::Sender<Event>) -> PlatformImplResult<Self> {
        Ok(PlatformImpl {})
    }

    pub fn create_window(&mut self, info: WindowInitialInfo) -> PlatformImplResult<Window> {
        // SAFETY:
        unsafe {
            binding::create_window();
        }
        let handler = WindowHandler::new().unwrap();
        Ok(Window::new(handler))
    }
}
