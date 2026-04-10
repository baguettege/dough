use ast::Node;
use ast::untyped::{Item, Func};
use crate::resolver::{ty, Resolver};
use crate::Result;
use crate::symbol::Symbol;

impl Resolver {
    pub(super) fn resolve_item(&mut self, item: &Item) -> Result<()> {
        match item {
            Item::Func(node) => self.resolve_func(node),
        }
    }

    fn resolve_func(&mut self, node: &Func) -> Result<()> {
        // global fn symbol already defined by collector
        self.with_scope(|this| {
            for param in node.params() {
                let ty = ty::resolve(param.ty())?;
                let symbol = Symbol::Local { id: param.id(), ty };
                
                this.define(param, param.ident(), symbol)?;
            }

            this.resolve_block(node.body())?;
            Ok(())
        })
    }
}
