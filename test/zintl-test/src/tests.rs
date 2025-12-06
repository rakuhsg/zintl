#[cfg(test)]
mod test {

    use zintl::*;
    use zintl_state::*;

    use crate::{runner::*, views::*};
    /*
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
    */
    #[test]
    fn stateful() {
        let app = App::new(TestStack::new().children(v![StatefulView::new(
            marked!(),
            || String::from("Hi"),
            |value| {
                println!("rerender");
                v![
                    TestStack::new().children(v![TestButton::new(value.clone()).on_click(|| {
                        println!("Click event triggered.");
                        *value = String::from("Clicked");
                    }),]),
                    StatefulView::new(
                        marked!(),
                        || String::from("Hoi"),
                        |value_child| {
                            v![TestButton::new(value_child.clone()).on_click(move || {
                                println!("Click event triggered in c.");
                                *value_child = "Clicked".to_string()
                            })]
                        }
                    )
                ]
            }
        )]));
        let mut runner = Runner::new(app);
        assert_eq!(runner.render(TestEvent::RedrawRequested), "HiHoi");
        assert_eq!(runner.render(TestEvent::Click), "HiHoi");
        assert_eq!(runner.render(TestEvent::RedrawRequested), "ClickedClicked");
        assert_eq!(runner.render(TestEvent::RedrawRequested), "ClickedClicked");
    }
}
