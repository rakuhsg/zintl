use crate::query::Query;

pub struct ModelContext {}

impl ModelContext {
    pub fn new() -> Self {
        ModelContext {}
    }
}

pub trait Model<Q: Query> {
    fn on(&mut self, q: Q, cx: &ModelContext);
    fn children(&self) -> impl IntoChildren;
}
