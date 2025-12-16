use std::sync::mpsc;

use crate::{
    driver::appkit::binding::runloop_run,
    event::{Event, ExitCode, RunMode},
};

pub struct EventDispatcher {
    mode: RunMode,
    receiver: mpsc::Receiver<Event>,
    sender: mpsc::Sender<Event>,
}

impl EventDispatcher {
    pub fn new(mode: RunMode) -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            mode,
            receiver,
            sender,
        }
    }

    pub fn get_sender(&self) -> mpsc::Sender<Event> {
        self.sender.clone()
    }

    pub fn dispatch_events(&self) -> Event {
        unsafe {
            runloop_run();
        }
        Event::None
    }
}
