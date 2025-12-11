//! Creating and handling native windows.
//!
//! # Features
//!
//!
//!
//! # Interoperability
//!
//! `wnd` supports [raw-window-handle](https://github.com/rust-windowing/raw-window-handle). It provides *standard types for accessing a window's platform-specific raw window handle and display's platform-specific raw display handle*.

mod driver;
pub mod event;
pub mod platform;
pub mod prelude;
pub mod window;
