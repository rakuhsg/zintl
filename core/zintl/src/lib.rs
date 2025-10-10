pub use zintl_ui::*;
// TODO: Rename to zintl_macro
pub use zintl_ui_macro::*;

#[macro_export]
macro_rules! v {
    [ $( $x:expr ),* $(,)? ] => {
        {
            fn assert_implements_view<E: ::zintl::Event, T: ::zintl::View<E>>(_v: &T) {}
            fn make_generator<E: ::zintl::Event, T: ::zintl::View<E> + 'static>(v: ::std::sync::Arc<::std::sync::Mutex<T>>) -> ::zintl::Generator<E> {
                Box::new(move |arena, storage, event| {
                    v.lock().expect("").render(arena, storage, event)
                })
            }

            let mut v = Vec::new();
            $(
                v.push(make_generator(::std::sync::Arc::new(::std::sync::Mutex::new($x))));
            )*
            v
        }
    };
}

#[macro_export]
macro_rules! marked {
    () => {
        format!("{}{}{}{}", module_path!(), file!(), line!(), column!())
    };
}
