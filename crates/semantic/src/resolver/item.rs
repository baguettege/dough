use ast::untyped::{Item, Fn, Static};
use crate::resolver::{ty, Resolver};
use crate::Result;
use crate::symbol::Symbol;

impl Resolver {
    pub(super) fn resolve_item(&mut self, item: &Item) -> Result<()> {
        match item {
            Item::Fn(node) => self.resolve_fn(node),
            Item::Static(node) => self.resolve_static(node),
        }
    }

    fn resolve_fn(&mut self, node: &Fn) -> Result<()> {
        // global fn symbol already defined by collector
        self.with_scope(|this| {
            for param in node.params() {
                let ty = ty::resolve(param.ty())?;
                let symbol = Symbol::Local(ty);

                // params are locally scoped only
                this.scope.define(param.ident(), symbol);
            }

            this.resolve_block(node.body())?;
            Ok(())
        })
    }

    fn resolve_static(&mut self, node: &Static) -> Result<()> {
        // must resolve the initializer before defining the variable,
        // otherwise declarations like `static x: int = x;` are possible
        self.resolve_expr(node.init())?;

        let ty = ty::resolve(node.ty())?;
        let symbol = Symbol::Global(ty);
        self.define(node, node.ident(), symbol)?;

        Ok(())
    }
}
