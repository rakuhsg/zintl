#[cfg(target_os = "macos")]
mod appkit;
mod types;
#[cfg(target_os = "windows")]
mod win32;
#[cfg(target_os = "macos")]
pub(crate) use appkit::implements::event::EventDispatcher;
#[cfg(target_os = "macos")]
pub(crate) use appkit::implements::platform::PlatformImpl;
#[cfg(target_os = "macos")]
pub(crate) use appkit::implements::window::WindowHandler;
pub(crate) use types::*;
#[cfg(target_os = "windows")]
pub(crate) use win32::implements::event::EventDispatcher;
#[cfg(target_os = "windows")]
pub(crate) use win32::implements::platform::PlatformImpl;
#[cfg(target_os = "windows")]
pub(crate) use win32::implements::window::NativeWindow as WindowHandler;
