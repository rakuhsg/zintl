use std::sync::{Arc, RwLock, RwLockReadGuard};

use zintl_ui::{Context, Event, Generator, ROArena, RenderNode, Storage, View};

#[derive(Clone)]
pub struct State<T: 'static> {
    value: Arc<RwLock<T>>,
}

impl<T: 'static> State<T> {
    pub fn new(value: T) -> Self {
        State {
            value: Arc::new(value.into()),
        }
    }

    pub fn value(&mut self) -> RwLockReadGuard<'_, T> {
        self.value.read().unwrap()
    }

    fn val_arc(&mut self) -> Arc<RwLock<T>> {
        self.value.clone()
    }

    pub fn set(&mut self, new: T) {
        let mut val = self.value.write().unwrap();
        *val = new;
    }
}

pub trait StatefulGenerator<E: Event, T: 'static>
where
    Self: FnMut(&mut State<T>) -> Vec<Generator<E>>,
{
}

impl<E: Event, T: 'static, G: FnMut(&mut State<T>) -> Vec<Generator<E>>> StatefulGenerator<E, T>
    for G
{
}

pub struct StatefulView<E: Event, T: 'static, G: StatefulGenerator<E, T>> {
    context: Context<E>,
    key: String,
    state: State<T>,
    generator: G,
}

impl<E: Event, T: 'static, G: StatefulGenerator<E, T>> std::fmt::Debug for StatefulView<E, T, G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StatefulView {{ ... }}")
    }
}

impl<E: Event, T: 'static, G: StatefulGenerator<E, T>> StatefulView<E, T, G> {
    pub fn new<F: Fn() -> T>(key: String, initial: F, generator: G) -> Self {
        StatefulView {
            context: Context::new(),
            key,
            state: State::new(initial()),
            generator,
        }
    }
}

impl<E: Event, T: 'static, G: StatefulGenerator<E, T>> View<E> for StatefulView<E, T, G> {
    fn get_context(&self) -> &Context<E> {
        &self.context
    }

    fn render(&mut self, arena: &mut ROArena, storage: &mut Storage, event: E) -> RenderNode {
        storage.insert(self.key.clone(), self.state.val_arc());
        let children = (self.generator)(&mut self.state);
        <StatefulView<E, T, G> as View<E>>::get_context(self).set_children(children);
        let mut node = RenderNode::empty();
        let child = <StatefulView<E, T, G> as View<E>>::get_context(self)
            .render_children(arena, storage, event);
        node.push_child(child);
        node
    }
}
