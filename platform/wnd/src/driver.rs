pub mod appkit;
pub mod error;
pub mod win32;

#[cfg(target_os = "windows")]
pub use win32::implements::window::NativeWindow as WindowHandler;

#[cfg(target_os = "windows")]
pub(crate) use win32::implements::event::EventDispatcher;

#[cfg(target_os = "windows")]
pub(crate) use win32::implements::platform::PlatformImpl;

#[cfg(target_os = "windows")]
pub(crate) use win32::types::*;
