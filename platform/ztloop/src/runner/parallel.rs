use std::{
    future::Future,
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, SyncSender, sync_channel},
    },
    task::Context,
    time::Duration,
};

use futures::{
    future::{BoxFuture, FutureExt},
    task::{ArcWake, waker_ref},
};

pub struct ParallelTaskRunner {}

impl ParallelTaskRunner {
    pub fn new() -> Self {
        ParallelTaskRunner {}
    }

    pub fn post_async(&self, marker: String, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
    }
}
