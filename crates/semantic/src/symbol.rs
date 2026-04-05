use std::collections::HashMap;
use ast::types::Ident;
use dough_core::Type;

#[derive(Debug)]
pub enum Symbol {
    Global(Type),
    Fn {
        params: Vec<Type>,
        return_ty: Type,
    },
}

#[derive(Default, Debug)]
pub struct Table {
    symbols: HashMap<Ident, Symbol>,
}

impl Table {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn insert(&mut self, key: Ident, symbol: Symbol) {
        self.symbols.insert(key, symbol);
    }

    pub fn lookup(&self, key: &Ident) -> Option<&Symbol> {
        self.symbols.get(key)
    }

    pub fn contains(&self, key: &Ident) -> bool {
        self.symbols.contains_key(key)
    }
}
