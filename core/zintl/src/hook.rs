#[derive(Clone, Copy, Debug)]
pub struct HookId(usize);

impl HookId {
    pub fn new(id: usize) -> Self {
        HookId(id)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

pub trait Hook {
    type Message;
    fn set_id(&mut self, id: HookId);
    fn get_id(&self) -> HookId;
    fn handle_message(&mut self, cx: &mut HookContext, message: Self::Message);
}

pub struct HookContext {
    triggered: Vec<HookId>,
}

impl HookContext {
    pub fn new() -> Self {
        HookContext {
            triggered: Vec::new(),
        }
    }

    pub fn trigger(&mut self, id: HookId) {
        self.triggered.push(id);
    }
}

pub struct HookManager<M> {
    hooks: Vec<Box<dyn Hook<Message = M>>>,
}
