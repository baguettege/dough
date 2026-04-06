use crate::{Error, Result};
use crate::resolver::Resolver;
use crate::symbol::Symbol;
use ast::Node;
use ast::untyped::Block;

impl Resolver {
    pub(super) fn with_scope<T, F>(&mut self, f: F) -> Result<T>
    where
        F: FnOnce(&mut Self) -> Result<T>
    {
        self.scope.enter();
        let result = f(self);
        self.scope.exit();
        result
    }
    
    pub(super) fn define(
        &mut self,
        node: &impl Node,
        ident: impl Into<String>,
        symbol: Symbol,
    ) -> Result<()> {
        let ident = ident.into();

        // lookup err = doesn't exist already
        if self.scope.lookup(&ident).is_err() {
            self.scope.define(ident, symbol.clone());
            self.bindings.insert(node, symbol);
            Ok(())
        } else {
            Err(Error::AlreadyDefined(ident))
        }
    }

    pub(super) fn resolve_block(&mut self, block: &Block) -> Result<()> {
        block.iter().try_for_each(|s| self.resolve_stmt(s))
    }
}
