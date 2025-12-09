use std::sync::mpsc::{channel, Receiver, Sender};

use crate::driver::{EventDispatcher, PlatformImpl, PlatformImplError};
use crate::event::{Event, RunMode};
use crate::window::{Window, WindowInitialInfo};

pub enum PlatformError {
    ImplError(PlatformImplError),
}

pub type PlatformResult<T> = Result<T, PlatformError>;

pub struct Platform {
    imp: PlatformImpl,
    evd: EventDispatcher,
}

impl Platform {
    pub fn new(mode: RunMode) -> PlatformResult<Self> {
        let evd = EventDispatcher::new(mode);
        let imp = PlatformImpl::new(evd.get_sender())?;
        Ok(Platform { imp, evd })
    }

    pub fn create_window(&mut self, info: WindowInitialInfo) -> Window {
        let sender = evd.get_sender();
    }

    pub fn dispatch(&self) -> Event {
        self.evd.dispatch_events()
    }
}
