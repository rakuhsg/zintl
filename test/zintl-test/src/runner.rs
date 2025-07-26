use zintl::*;

pub enum AppError {}

pub enum Event {
    Click,
    PressAKey,
    PressBKey,
}

pub struct Runner {
    app: App,
}

impl Runner {
    pub fn new(app: App) -> Self {
        Runner { app }
    }

    pub fn render(&mut self) -> String {
        let mut result = String::new();

        let mut objects = vec![];
        let node = self.app.root();
        recursively_get_render_objects(&node, &mut objects);

        for obj in objects {
            match obj.content {
                RenderContent::Text(text) => {
                    result = result + &text;
                }
                _ => {}
            }
        }

        result
    }

    pub fn fire_event(&mut self, _event: Event) {}
}

fn recursively_get_render_objects(node: &RenderNode, objects: &mut Vec<RenderObject>) {
    objects.push(node.object.clone());
    if let Some(node) = &node.inner {
        recursively_get_render_objects(node.as_ref(), objects);
    }
    for child in &node.children {
        recursively_get_render_objects(&child, objects);
    }
}
