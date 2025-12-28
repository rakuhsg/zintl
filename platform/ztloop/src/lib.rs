//! ZTLoop is a task runner / async executor implementation for handling UI
//! events. ## Design
//!
//! ### Tasks
//!
//! Tasks are like closures that don't poll.
//!
//! ```
//! When you call `runner.post(here!(), some_task);`...
//!
//! [ Task Runner ]
//! ↓ Passes the task to Dispatcher.
//! [ Schedular ] (Push your task into a queue.)
//! ↓ Use OSLoop to schedule tasks in sequence.
//! [ User Interface OSLoop ]
//! ↓
//! Run a task.
//! ```
//!
//! ```rust
//! let task = Task::new(async {
//! 	do_something().await;
//! }, TaskPriorityHint::Immidiately);
//!
//! parallel_runner.post(here!(), task);
//! ```
//! ### Task Runner
//!
//! Queues and runs tasks with `Schedular`
//!
//! ### Schedular
//!
//! Prioritizing and scheduling tasks, dispatching events from `OSLoop` and
//! managing timers.
//!
//! ### OSLoop
//!
//! `OSLoop` handles platform-specific event loop and message pump system like
//! `CFRunLoop` on macOS with Appkit and `PollMessage/Wndproc` on Windows.
//! `OSLoop`
//!
//! ## Task Runners
//!
//! ### ParallelTaskRunner
//!
//! Executing asynchronous tasks with `post_async`.
//!
//! ### SequencedTaskRunner
//!
//! Executing tasks in sequence.
//!
//! Note: Async/Await is not supported on `SequencedTaskRunner`.

pub(crate) mod implementation;
pub mod osloop;
pub mod runner;
