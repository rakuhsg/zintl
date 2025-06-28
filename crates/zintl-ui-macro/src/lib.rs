#[allow(unused)]
use zintl_ui_view::{Generator, Storage, View};

#[allow(unused)]
fn assert_implements_view<T: View>(_v: &T) {}

#[macro_export]
macro_rules! v {
    [ $( $x:expr ),* $(,)? ] => {
        {
            let mut v: Vec<Generator> = Vec::new();
            $(
                let a = $x;
                assert_implements_view(&a);
                v.push(Box::new(move |storage| {
                    a.render(storage)
                }));
            )*
            v
        }
    };
}
