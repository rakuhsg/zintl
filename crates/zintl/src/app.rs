use crate::render::{Metrics, Position, RenderContent, RenderObject};

pub struct App {
    root: RenderObject,
}

impl App {
    pub fn new(root: impl View) -> Self {
        App {
            root: root.get_context().render(),
        }
    }

    pub fn get_render_object(&self) -> RenderObject {
        self.root.clone()
    }
}

/// The context consists of a set of style properties and layouts to render views.
pub struct Context {
    pub render_object: RenderObject,
}

impl Context {
    pub fn new() -> Self {
        Context {
            render_object: RenderObject::default(),
        }
    }
    pub fn from_render_object(render_object: RenderObject) -> Self {
        Context { render_object }
    }
    pub fn set_style_property(&self) {}
    pub fn render(&self) -> RenderObject {
        self.render_object.clone()
    }
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
        let context = Context::from_render_object(RenderObject::new(
            RenderContent::Text(text.to_string()),
            Position::new(0., 0.),
            Metrics::Auto,
        ));
        Label {
            text: text.to_string(),
            context,
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
