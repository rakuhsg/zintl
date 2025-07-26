#[cfg(test)]
mod test {
    use crate::runner::Runner;
    use crate::views::TestLabel;
    use zintl::App;

    #[test]
    fn custom_hello_world() {
        let app = App::new(TestLabel::new("hello, world!".to_string()));
        let mut runner = Runner::new(app);
        assert_eq!(runner.render(), "hello, world!");
    }
}
