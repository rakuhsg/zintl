/// ```ignore
/// struct Counter {
///     count: i32,
///     view: View,
/// }
///
/// impl Counter {
///     fn new() -> Self {
///         Counter {
///             count: 0,
///             view: View::new(),
///         }
///     }
/// }
///
/// impl HasView {
///     fn view(&mut self) -> View {
///         self.view.with(| children | {
///             vstack()
///                 .child(label(format!("{}", self.count)))
///                 .child(button("count").on_click(|e| {
///                     self.count += 1;
///                 }))
///                 .child(vstack().children(children))
///         })
///     }
/// }
///
/// fn main() {
///     App::new().render(
///         Counter::new().padding(10, 10, 10, 10)
///     )
/// }
/// ```
///
pub mod app;
