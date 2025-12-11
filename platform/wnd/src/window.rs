use raw_window_handle::{HandleError, HasWindowHandle, WindowHandle};

use crate::driver;

pub struct Window {
    handler: driver::WindowHandler,
}

#[derive(Debug)]
pub enum WindowError {
    ImplError(driver::WHImplError),
}

pub type WindowResult<T> = Result<T, WindowError>;

pub struct WindowInitialInfo {
    pub pos_x: i32,
    pub pos_y: i32,
    pub width: i32,
    pub height: i32,
    pub title: String,
}

impl Window {
    pub(crate) fn new(handler: driver::WindowHandler) -> Self {
        Window { handler }
    }

    pub fn apply_system_appearance(&self) {
        self.handler.apply_system_appearance();
    }

    pub fn set_title(&self, title: &str) {
        self.handler.set_title(title);
    }

    pub fn get_title(&self) {
        self.handler.get_title()
    }
}

impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let raw = self.handler.rwh()?;

        Ok(unsafe { WindowHandle::borrow_raw(raw) })
    }
}
