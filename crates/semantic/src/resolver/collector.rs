use crate::resolver::{ty, Resolver};
use crate::symbol::Symbol;
use crate::Result;
use ast::untyped::{Fn, Item, Program};
use dough_core::Type;

struct Collector<'a> {
    resolver: &'a mut Resolver,
}

impl<'a> Collector<'a> {
    fn new(resolver: &'a mut Resolver) -> Self {
        Self { resolver }
    }

    fn collect(mut self, program: &Program) -> Result<()> {
        program.iter().try_for_each(|i| self.collect_item(i))
    }

    fn collect_item(&mut self, item: &Item) -> Result<()> {
        match item {
            Item::Fn(node) => self.collect_fn(node),
            Item::Static(_) => Ok(()), // skip - handled in resolve pass
        }
    }

    fn collect_fn(&mut self, node: &Fn) -> Result<()> {
        let params = node.params()
            .iter()
            .map(|p| ty::resolve(p.ty()))
            .collect::<Result<Vec<_>>>()?;
        let return_ty = node.return_ty()
            .as_ref()
            .map(ty::resolve)
            .transpose()?
            .unwrap_or(Type::Unit);

        let symbol = Symbol::Fn { params, return_ty };
        self.resolver.define(node, node.ident(), symbol)?;

        Ok(())
    }
}

pub(super) fn collect(resolver: &mut Resolver, program: &Program) -> Result<()> {
    Collector::new(resolver).collect(program)
}
