use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

use crate::render::{Metrics, Position, RenderContent, RenderObject};

#[derive(Clone, Debug)]
pub struct Storage {
    data: Arc<HashMap<String, Arc<dyn Any>>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            data: HashMap::new().into(),
        }
    }
}

pub struct ViewBuilder {}

#[derive(Clone, Debug)]
pub struct App<T>
where
    T: Fn(&mut Storage) -> ViewBuilder,
{
    storage: Storage,
    generator: T,
}

impl<T: Fn(&mut Storage) -> ViewBuilder> App<T> {
    pub fn new(generator: T) -> Self {
        let mut storage = Storage::new();
        App { storage, generator }
    }

    pub fn get_render_object(&self) -> RenderObject {
        todo!()
        //self.root.clone()
    }
}

/// The context consists of a set of style properties and layouts to render views.
#[derive(Default)]
pub struct Context {
    pub render_object: RenderObject,
    pub storage: Option<Arc<HashMap<String, Arc<dyn Any>>>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            render_object: RenderObject::default(),
            storage: None,
        }
    }
    pub fn set_style_property(&self) {}
    pub fn insert_pending_storage<T>(&mut self, key: String, value: T) {}
    pub fn render(&self) -> RenderObject {
        self.render_object.clone()
    }
}

/// A renderable component that has context.
pub trait View: Sized {
    fn get_context(&self) -> &Context;

    // TODO
    #[allow(unused)]
    fn padding(self, top: f32, bottom: f32, left: f32, right: f32) -> Self {
        self.get_context().set_style_property();
        self
    }
}

/// A component uses other components to compose its view.
pub trait Composable: Sized {
    fn view(&mut self) -> impl View;
}

/// A view that implements the [`Composable`] trait.
pub trait ComposableView: Sized {
    fn context(&self) -> &Context;
    fn compose(&mut self) -> impl View;

    fn children<const N: usize>(self, children: [impl View; N]) -> Self {
        let func = move || println!("{children:?}");
        for child in children {
            self.context()
                .render_object
                .add_child(child.get_context().render());
        }
        self
    }
}

impl<T: ComposableView> Composable for T {
    fn view(&mut self) -> impl View {
        self.compose()
    }
}

impl<T: ComposableView> View for T {
    fn get_context(&self) -> &Context {
        let view = self.view();
        self.context()
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
    // TODO
    #[allow(dead_code)]
    text: String,
    context: Context,
}

impl Label {
    pub fn new(text: String) -> Self {
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
    fn compose(&mut self) -> impl View {}
}

pub struct Button {
    context: Context,
}

impl Button {
    pub fn new() -> Self {
        Button {
            context: Context::new(),
        }
    }
}

impl ComposableView for Button {
    fn context(&self) -> &Context {
        &self.context
    }
    fn compose(&mut self) -> impl View {
        Stack::new()
    }
}
