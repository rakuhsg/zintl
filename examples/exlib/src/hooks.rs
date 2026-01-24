use zintl::hook::{Hook, HookContext, HookId};

use crate::{event::SystemMessage, render::Rect};

#[derive(Copy, Clone, Debug)]
pub struct ClickEventHook {
    id: HookId,
    covered: Rect,
}

impl Hook for ClickEventHook {
    type Message = SystemMessage;

    fn init(&mut self, id: HookId) {
        self.id = id;
    }

    fn get_id(&self) -> HookId {
        self.id
    }

    fn handle_message(&mut self, cx: &mut HookContext, message: SystemMessage) {
        match message {
            SystemMessage::MouseClick { x, y } => {
                let cov = self.covered;
                if cov.min_x <= x && cov.min_y <= y {
                    if cov.max_x >= x && cov.max_y >= y {
                        cx.trigger(self.id);
                    }
                }
            }
            _ => {}
        }
    }
}
