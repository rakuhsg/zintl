use zintl::app::{self, App, ComposableView, Context, Label, Stack, View};
use zintl_render::run_app;

struct Counter {
    count: i32,
    context: Context,
}

impl Counter {
    fn new() -> Self {
        Counter {
            count: 0,
            context: Context::new(),
        }
    }
}

impl ComposableView for Counter {
    fn context(&self) -> &Context {
        &self.context
    }
    fn compose(&mut self) -> impl View {
        Stack::new()
    }
}

fn main() {
    let app = App::new(Counter::new().padding(10., 10., 10., 10.).children([
        Stack::new().padding(10., 10., 10., 10.).children([
            Label::new("hi"),
            Label::new("hello"),
            Label::new("world").children([Label::new("hi")]),
        ]),
    ]));

    run_app(app);
}
