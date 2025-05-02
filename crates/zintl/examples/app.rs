use zintl::app::{self, App, ComposableView, Context, Label, Stack, View};
use zintl_render::run_app;

/*struct Counter {
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
}*/

fn main() {
    let app = App::new(Label::new("Hello, World!"));

    run_app(app);
}
