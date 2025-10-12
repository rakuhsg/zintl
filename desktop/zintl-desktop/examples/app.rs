use zintl::*;
use zintl_desktop::*;
use zintl_widget::*;

struct HelloWorld {
    context: Context,
}

impl HelloWorld {
    fn new() -> Self {
        HelloWorld {
            context: Context::default(),
        }
    }
}

impl Composable for HelloWorld {
    fn context(&self) -> &Context {
        &self.context
    }

    fn compose(&mut self) -> impl View {
        Stack::new().children(v![Label::new("Hellope".to_string()),])
    }
}

fn main() {
    let app = App::new(HelloWorld::new());

    run_app(app);
}
