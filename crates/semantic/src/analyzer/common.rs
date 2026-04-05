use ast::typed::{Block, Expr};
use ast::types::Ident;
use ast::untyped;
use dough_core::Type;
use crate::analyzer::{resolve, Analyzer};
use crate::{Error, Result};
use crate::symbol::Symbol;

impl Analyzer {
    pub(super) fn return_ty(&self) -> Type {
        self.return_ty.expect("return_ty unset outside fn")
    }

    pub(super) fn analyze_block(&mut self, block: &untyped::Block) -> Result<Block> {
        block
            .iter()
            .map(|stmt| self.analyze_stmt(stmt))
            .collect()
    }

    pub(super) fn with_scope<T, F>(&mut self, f: F) -> Result<T>
    where
        F: FnOnce(&mut Self) -> Result<T>
    {
        self.stack.push();
        let result = f(self);
        self.stack.pop();
        result
    }
    
    pub(super) fn lookup_var(&mut self, ident: &Ident) -> Result<Type> {
        self.stack.lookup(ident)
            .or_else(|| match self.table.lookup(ident) {
                Some(Symbol::Global(ty)) => Some(*ty),
                _ => None,
            })
            .ok_or_else(|| Error::UndefinedIdent(ident.clone()))
    }
}

pub(super) fn expect_type(expected: Type, found: Type) -> Result<()> {
    if expected == found {
        Ok(())
    } else {
        Err(Error::TypeMismatch { expected, found })
    }
}

pub(super) fn expect_expr_type(expr: &Expr, expected: Type) -> Result<()> {
    expect_type(expected, resolve::expr(expr))
}
