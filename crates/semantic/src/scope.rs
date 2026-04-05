use std::collections::HashMap;
use ast::types::Ident;
use dough_core::Type;

#[derive(Default)]
pub(crate) struct Stack {
    scopes: Vec<HashMap<Ident, Type>>,
}

impl Stack {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn push(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub(crate) fn pop(&mut self) {
        self.scopes.pop();
    }

    pub(crate) fn insert(&mut self, key: Ident, ty: Type) {
        self.scopes
            .last_mut()
            .expect("scope stack underflow")
            .insert(key, ty);
    }

    pub(crate) fn lookup(&self, key: &Ident) -> Option<Type> {
        self.scopes
            .iter()
            .rev()
            .find_map(|s| s.get(key))
            .copied()
    }
}
