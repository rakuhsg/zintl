use crate::driver::types::*;

pub struct WindowHandler {}

impl WindowHandler {
    pub fn new() -> WHImplResult<Self> {
        Ok(WindowHandler {})
    }

    pub fn set_title(&self, _title: String) {}

    pub fn get_title(&self) {}

    pub fn apply_system_appearance(&self) {}

    pub fn rwh(
        &self,
    ) -> Result<raw_window_handle::RawWindowHandle, raw_window_handle::HandleError> {
        unimplemented!()
    }
}
