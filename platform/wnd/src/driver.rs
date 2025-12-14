mod appkit;
mod types;
mod win32;
pub(crate) use types::*;
#[cfg(target_os = "windows")]
pub(crate) use win32::implements::event::EventDispatcher;
#[cfg(target_os = "windows")]
pub(crate) use win32::implements::platform::PlatformImpl;
#[cfg(target_os = "windows")]
pub use win32::implements::window::NativeWindow as WindowHandler;
