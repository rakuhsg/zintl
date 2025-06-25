use zintl_run::run_app;
use zintl_ui::app::{App, Button, ComposableView, Context, Label, Stack, View};

struct Counter {
    count: usize,
    context: Context,
}

impl Counter {
    fn new() -> Self {
        Self {
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
        Stack::new().children([
            Label::new(format!("{}", self.count)),
            Button::new("Increment"),
        ])
    }
}

fn main() {
    let app = App::new(Counter::new());

    run_app(app);
}
