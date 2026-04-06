use std::collections::HashMap;
use crate::symbol::Symbol;
use crate::{Result, Error};

#[derive(Default)]
pub(super) struct Scope {
    scopes: Vec<HashMap<String, Symbol>>,
}

impl Scope {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn enter(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub(super) fn exit(&mut self) {
        self.scopes
            .pop()
            .expect("scope underflow: exit called without matching enter");
    }

    pub(super) fn define(&mut self, ident: impl Into<String>, symbol: Symbol) {
        self.scopes
            .last_mut()
            .expect("scope underflow: define called without a scope")
            .insert(ident.into(), symbol);
    }

    pub(super) fn lookup(&self, ident: &str) -> Result<&Symbol> {
        self.scopes
            .iter()
            .rev()
            .find_map(|sc| sc.get(ident))
            .ok_or_else(|| Error::UndefinedIdent(ident.into()))
    }
}
