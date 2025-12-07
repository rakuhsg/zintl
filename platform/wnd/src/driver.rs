pub mod appkit;
pub mod error;
pub mod win32;

#[cfg(target_os = "windows")]
pub use win32::implements::NativeWindow as WindowHandler;

#[cfg(target_os = "windows")]
pub(crate) use win32::implements::EventRunner;
