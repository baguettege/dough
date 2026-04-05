use std::collections::HashMap;
use ast::types::Ident;
use dough_core::Type;

/// Local variable scope stack, used internally during analysis to
/// track variable types within function bodies. Locals are a codegen's
/// concern, therefore, this is not a part of the public output.
#[derive(Default)]
pub(crate) struct Stack {
    scopes: Vec<HashMap<Ident, Type>>,
}

impl Stack {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Enters a new scope.
    pub(crate) fn push(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Exits the current scope.
    pub(crate) fn pop(&mut self) {
        self.scopes.pop();
    }

    /// Inserts a local variable into the current scope.
    pub(crate) fn insert(&mut self, ident: Ident, ty: Type) {
        self.scopes
            .last_mut()
            .expect("scope stack underflow")
            .insert(ident, ty);
    }

    /// Looks up a local variable by name, searching from the innermost
    /// to the outermost scope.
    pub(crate) fn lookup(&self, ident: &Ident) -> Option<Type> {
        self.scopes
            .iter()
            .rev()
            .find_map(|s| s.get(ident))
            .copied()
    }
}
