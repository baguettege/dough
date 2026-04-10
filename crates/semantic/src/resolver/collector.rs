use ast::Node;
use crate::resolver::{ty, Resolver};
use crate::symbol::Symbol;
use crate::Result;
use ast::untyped::{Func, Item, Program};
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
            Item::Func(node) => self.collect_func(node),
        }
    }

    fn collect_func(&mut self, node: &Func) -> Result<()> {
        let params = node.params()
            .iter()
            .map(|p| ty::resolve(p.ty()))
            .collect::<Result<Vec<_>>>()?;
        let return_ty = node.return_ty()
            .as_ref()
            .map(ty::resolve)
            .transpose()?
            .unwrap_or(Type::Unit);
        let id = node.id();

        let symbol = Symbol::Func { params, return_ty, id };
        self.resolver.define(node, node.ident(), symbol)?;

        Ok(())
    }
}

pub(super) fn collect(resolver: &mut Resolver, program: &Program) -> Result<()> {
    Collector::new(resolver).collect(program)
}
