use zintl_run::run_app;
use zintl_ui::app::{App, Label};

fn main() {
    let app = App::new(Label::new("Hello, World!"));

    run_app(app);
}
