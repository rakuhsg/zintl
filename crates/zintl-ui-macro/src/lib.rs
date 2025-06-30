#[allow(unused)]
use zintl_ui_view::{Generator, Storage, View};

#[macro_export]
macro_rules! v {
    [ $( $x:expr ),* $(,)? ] => {
        {
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
