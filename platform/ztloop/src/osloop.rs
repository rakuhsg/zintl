#[cfg(target_os = "macos")]
pub use crate::implementation::appkit::AppkitEventPump as EventPumpImpl;

/// [`EventPump`] is an abstraction of a native message loop.
/// NOTE: Commonly, only one [`EventPump`] can exist at a time. After
/// [`EventPump`] drops, you can initialize EventPump again.
/// NOTE: Commonly, implementations are !Send !Sync.
pub trait EventPump {
    /// Run event loop.
    /// NOTE: Must be called on main thread.
    fn run(&mut self);
    /// Exit the event loop immidiately.
    /// NOTE: Must be called on main thread.
    fn quit(&mut self);
}
