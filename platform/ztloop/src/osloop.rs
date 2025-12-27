#[cfg(target_os = "macos")]
pub use crate::implementation::appkit::AppkitOSLoop as OSLoopImpl;

/// [`OSLoop`] is an abstraction of a native message loop.
/// NOTE: Commonly, only one [`OSLoop`] can exist at a time on the thread. After
/// [`OSLoop`] drops, you can initialize OSLoop again.
/// NOTE: Commonly, implementations are !Send !Sync.
pub trait OSLoop {
    /// Run event loop.
    fn run(&mut self);
    /// Exit the event loop immidiately.
    /// NOTE: Must be called on the thread that run() called before.
    fn quit(&mut self);
}
