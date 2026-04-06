use crate::resolver::scope::Scope;
use crate::{bindings, Result};
use ast::untyped::Program;

mod scope;
mod ty;
mod item;
mod stmt;
mod expr;
mod common;
mod collector;

#[derive(Default)]
struct Resolver {
    scope: Scope,
    bindings: bindings::Table,
}

impl Resolver {
    fn new() -> Self {
        Self::default()
    }

    fn resolve(mut self, program: &Program) -> Result<bindings::Table> {
        self.with_scope(|this| {
            collector::collect(this, program)?;
            program
                .iter()
                .try_for_each(|item| this.resolve_item(item))
        })?;

        Ok(self.bindings)
    }
}

pub(crate) fn resolve(program: &Program) -> Result<bindings::Table> {
    Resolver::new().resolve(program)
}
