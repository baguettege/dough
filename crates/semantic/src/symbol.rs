use std::collections::HashMap;
use ast::types::Ident;
use dough_core::Type;

/// A resolved top-level symbol. Only globals and functions are stored
/// here. Locals are tracked via [`crate::scope::Stack`], and tracked
/// in codegen via its own scope stack.
#[derive(Debug)]
pub enum Symbol {
    Global(Type),
    Fn {
        params: Vec<Type>,
        return_ty: Type,
    },
}

/// Maps top-level names to their resolved [`Symbol`]s.
/// Built during the first pass of semantic analysis and consumed
/// by codegen.
#[derive(Default, Debug)]
pub struct Table {
    symbols: HashMap<Ident, Symbol>,
}

impl Table {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Inserts a symbol. Only called during analysis - `pub(crate)` is intentional.
    pub(crate) fn insert(&mut self, ident: Ident, symbol: Symbol) {
        self.symbols.insert(ident, symbol);
    }

    /// Looks up a symbol by `ident`. Returns `None` if `ident` is not a
    /// top-level item. A `None` result in codegen means that it is a local.
    pub fn lookup(&self, ident: &Ident) -> Option<&Symbol> {
        self.symbols.get(ident)
    }
    
    pub fn contains(&self, ident: &Ident) -> bool {
        self.symbols.contains_key(ident)
    }
}
