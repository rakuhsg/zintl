#[cfg(test)]
mod test {
    use zintl::*;
    use zintl_state::*;

    use crate::{runner::*, views::*};

    #[test]
    fn custom_hello_world() {
        let app = App::new(TestLabel::new("hello, world!".to_string()));
        let mut runner = Runner::new(app);
        assert_eq!(runner.render(TestEvent::RedrawRequested), "hello, world!");
    }

    #[test]
    fn stack() {
        let app =
            App::new(TestStack::new().children(v![TestLabel::new("hello, world!".to_string())]));
        let mut runner = Runner::new(app);
        assert_eq!(runner.render(TestEvent::RedrawRequested), "hello, world!");
    }

    #[test]
    fn stateful() {
        let app = App::new(StatefulView::new(marked!(), "Hi".to_string(), |value| {
            let mut v = value.clone();
            v![
                TestStack::new().children(v![TestButton::new(value.value().to_string()).on_click(
                    move || {
                        println!("Click event triggered.");
                        v.set("Clicked".to_string().to_owned());
                    }
                ),])
            ]
        }));
        let mut runner = Runner::new(app);
        assert_eq!(runner.render(TestEvent::RedrawRequested), "Hi");
        assert_eq!(runner.render(TestEvent::Click), "Hi");
        assert_eq!(runner.render(TestEvent::RedrawRequested), "Clicked");
    }
}
