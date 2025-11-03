use crate::{
    event::Event,
    render::{ROArena, RenderNode},
    view::{Generator, Storage, View},
};

pub struct App<E: Event> {
    storage: Storage,
    pub ro_arena: ROArena,
    pub root: RenderNode,
    generator: Generator<E>,
    phantom: std::marker::PhantomData<E>,
}

impl<E: Event> App<E> {
    pub fn new(mut view: impl View<E> + 'static) -> Self {
        let mut storage = Storage::new();
        let mut ro_arena = ROArena::new();
        let root = view.render(&mut ro_arena, &mut storage, E::initial());
        let generator: Generator<E> =
            Box::new(move |arena, storage, event| view.render(arena, storage, event));
        App {
            storage,
            ro_arena,
            root,
            generator,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn root(&mut self) -> &RenderNode {
        &self.root
    }

    pub fn ro_arena(&mut self) -> &ROArena {
        &self.ro_arena
    }

    pub fn render(&mut self, event: E) {
        let storage = &mut self.storage;
        let arena = &mut self.ro_arena;
        let root = (self.generator)(arena, storage, event);
        self.root = root;
    }
}
