use zintl::*;

use crate::views::TestRenderObject;

pub enum AppError {}

#[derive(Copy, Clone, Debug)]
pub enum TestEvent {
    RedrawRequested,
    Click,
    PressAKey,
    PressBKey,
}

impl zintl::Event for TestEvent {
    fn initial() -> Self {
        TestEvent::RedrawRequested
    }
}

pub struct Runner {
    app: App<TestEvent>,
}

impl Runner {
    pub fn new(app: App<TestEvent>) -> Self {
        Runner { app }
    }

    pub fn render(&mut self, event: TestEvent) -> String {
        let mut result = String::new();

        let mut objects = vec![];
        self.app.render(event);
        let tree = &self.app.root;
        let arena = &self.app.ro_arena;
        recursively_get_render_objects(arena, tree, &mut objects);

        println!("{} render objects found", objects.len());

        for obj in objects {
            match obj {
                TestRenderObject::Text(text) => {
                    result += text;
                }
                _ => {}
            }
        }

        result
    }

    pub fn fire_event(&mut self, _event: TestEvent) {}
}

fn recursively_get_render_objects<'a>(
    arena: &'a ROArena,
    node: &RenderNode,
    objects: &mut Vec<&'a TestRenderObject>,
) {
    match arena.get_safe::<TestRenderObject>(node.object) {
        Some(object) => {
            objects.push(object);
        }
        _ => {}
    }
    for child in &node.children {
        recursively_get_render_objects(&arena, &child, objects);
    }
}
