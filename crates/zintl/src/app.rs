pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn render(&mut self, root: impl View) {}
}

/// The context consists of a set of style properties and layouts to render views.
pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Context {}
    }
    pub fn set_style_property(&self) {}
}

/// A renderable component that has context.
pub trait View: Sized {
    fn get_context(&self) -> &Context;

    fn padding(self, top: f32, bottom: f32, left: f32, right: f32) -> Self {
        self.get_context().set_style_property();
        self
    }
}

/// A component uses other components to compose its view.
pub trait Composable: Sized {
    fn compose_view(&mut self) -> impl View;
}

/// A view that implements the [`Composable`] trait.
pub trait ComposableView: Sized {
    fn context(&self) -> &Context;
    fn compose(&mut self) -> impl View;

    fn children<const N: usize>(self, children: [impl View; N]) -> Self {
        self
    }
}

impl<T: ComposableView> View for T {
    fn get_context(&self) -> &Context {
        self.context()
    }
}

impl<T: ComposableView> Composable for T {
    fn compose_view(&mut self) -> impl View {
        self.compose()
    }
}

/// The root component of views.
///
/// [`Base`] is not composable and has no children because it is the root of the view hierarchy.
pub struct Base {
    context: Context,
}

impl Base {
    fn new() -> Self {
        Self {
            context: Context::new(),
        }
    }
}

impl View for Base {
    fn get_context(&self) -> &Context {
        &self.context
    }
}

pub struct Stack {
    context: Context,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            context: Context::new(),
        }
    }
}

impl ComposableView for Stack {
    fn context(&self) -> &Context {
        &self.context
    }
    fn compose(&mut self) -> impl View {
        Base::new()
    }
}

pub struct Label {
    text: String,
    context: Context,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Label {
            text: text.to_string(),
            context: Context::new(),
        }
    }
}

impl ComposableView for Label {
    fn context(&self) -> &Context {
        &self.context
    }
    fn compose(&mut self) -> impl View {
        Stack::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Counter {
        count: i32,
        context: Context,
    }

    impl Counter {
        fn new() -> Self {
            Counter {
                count: 0,
                context: Context::new(),
            }
        }
    }

    impl ComposableView for Counter {
        fn context(&self) -> &Context {
            &self.context
        }
        fn compose(&mut self) -> impl View {
            Stack::new()
        }
    }

    #[test]
    fn app() {
        App::new().render(Counter::new().padding(10., 10., 10., 10.).children([
            Stack::new().padding(10., 10., 10., 10.).children([
                Label::new("hi"),
                Label::new("hello"),
                Label::new("world").children([Label::new("hi")]),
            ]),
        ]));
    }
}
