use std::marker::PhantomData;

use zintl_ui::{Context, Event, Generator, ROArena, RenderNode, Storage, View};

pub trait StateValue: 'static + Clone {}
impl<T> StateValue for T where T: 'static + Clone {}

pub trait StatefulGenerator<E: Event, T: StateValue, F>
where
    F: FnOnce() -> Vec<Generator<E>>,
    Self: FnMut(&'static mut T) -> F,
{
}

impl<E: Event, T: StateValue, F: FnOnce() -> Vec<Generator<E>>, G: FnMut(&'static mut T) -> F>
    StatefulGenerator<E, T, F> for G
{
}

pub struct StatefulView<E, T, F, G>
where
    E: Event,
    T: StateValue,
    F: FnOnce() -> Vec<Generator<E>>,
    G: StatefulGenerator<E, T, F>,
{
    context: Context<E>,
    key: String,
    initial_state: Box<T>,
    generator: G,
    phantom: PhantomData<F>,
}

impl<E, T, F, G> std::fmt::Debug for StatefulView<E, T, F, G>
where
    E: Event,
    T: StateValue,
    F: FnOnce() -> Vec<Generator<E>>,
    G: StatefulGenerator<E, T, F>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StatefulView {{ ... }}")
    }
}

impl<E, T, F, G> StatefulView<E, T, F, G>
where
    E: Event,
    T: StateValue,
    F: FnOnce() -> Vec<Generator<E>>,
    G: StatefulGenerator<E, T, F>,
{
    pub fn new<I: Fn() -> T>(key: String, initial: I, generator: G) -> Self {
        StatefulView {
            context: Context::new(),
            key,
            initial_state: Box::new(initial()),
            generator,
            phantom: PhantomData,
        }
    }
}

impl<E, T, F, G> View<E> for StatefulView<E, T, F, G>
where
    E: Event,
    T: StateValue,
    F: FnOnce() -> Vec<Generator<E>>,
    G: StatefulGenerator<E, T, F>,
{
    fn get_context(&self) -> &Context<E> {
        &self.context
    }

    fn render(&mut self, arena: &mut ROArena, storage: &mut Storage, event: E) -> RenderNode {
        let children = storage.modify(self.key.clone(), |v: Option<&mut T>| match v {
            Some(v) => {
                // Erasing lifetime
                //SAFETY: generator closure drops before this value will drops.
                let v: &'static mut T = unsafe { std::mem::transmute(v) };
                Some((self.generator)(v))
            }
            None => None,
        });
        match children {
            // Render child elm
            Some(children) =>
                <StatefulView<E, T, F, G> as View<E>>::get_context(self).set_children(children()),
            None => {
                // No entry found so we need to insert default value.
                storage.insert(self.key.clone(), self.initial_state.clone());
                // Try one more time.
                let children = storage.modify(self.key.clone(), |v: Option<&mut T>| match v {
                    Some(v) => {
                        // Erasing lifetime
                        //SAFETY: generator closure drops before this value will drops.
                        let v: &'static mut T = unsafe { std::mem::transmute(v) };
                        Some((self.generator)(v))
                    }
                    None => None,
                });
                match children {
                    Some(children) => <StatefulView<E, T, F, G> as View<E>>::get_context(self)
                        .set_children(children()),
                    None => {}
                }
            }
        }
        let mut node = RenderNode::empty();
        let child = <StatefulView<E, T, F, G> as View<E>>::get_context(self)
            .render_children(arena, storage, event);
        node.push_child(child);
        node
    }
}
