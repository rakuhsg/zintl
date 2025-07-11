#[macro_export]
macro_rules! v {
    [ $( $x:expr ),* $(,)? ] => {
        {
            #[allow(unused)]
            use zintl_ui::{Generator, Storage, View};

            fn assert_implements_view<T: View>(_v: &T) {}
            let mut v: Vec<Generator> = Vec::new();
            $(
                let mut a = $x;
                assert_implements_view(&a);
                v.push(Box::new(move |storage| {
                    a.render(storage)
                }));
            )*
            v
        }
    };
}
