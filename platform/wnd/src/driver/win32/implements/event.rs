use crate::event::{Event, ExitCode, RunMode};
use std::sync::mpsc;

use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, PeekMessageW, TranslateMessage, MSG, PM_REMOVE, WM_QUIT,
};

pub(crate) struct EventDispatcher {
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
        let mut msg = MSG::default();

        unsafe {
            if PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                let _ = TranslateMessage(&mut msg);
                DispatchMessageW(&mut msg);

                if msg.message == WM_QUIT {
                    return Event::Exit(ExitCode::Success);
                }
            }
        }
        Event::None
    }
}
