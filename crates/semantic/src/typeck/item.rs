use ast::typed::{Item, Fn, Static, Param, Block, Stmt};
use ast::{untyped, Node};
use dough_core::Type;
use crate::{Error, Result};
use crate::symbol::Symbol;
use crate::typeck::{ty, TypeChecker};

impl TypeChecker<'_> {
    pub(super) fn check_item(&mut self, item: &untyped::Item) -> Result<Item> {
        match item {
            untyped::Item::Fn(node) => self.check_fn(node).map(Into::into),
            untyped::Item::Static(node) => self.check_static(node).map(Into::into),
        }
    }

    fn check_fn(&mut self, node: &untyped::Fn) -> Result<Fn> {
        // resolver guarantees this is a `Symbol::Fn`
        let Symbol::Fn { params, return_ty } = self.bindings.get(node)
        else { unreachable!() };

        let params = params
            .iter()
            .zip(node.params())
            .map(|(&ty, param)|
                Param::new(param.ident().into(), ty))
            .collect::<Vec<_>>();

        self.return_ty = Some(*return_ty);
        let body = self.check_block(node.body());
        self.return_ty = None;
        let body = body?;

        let ident = node.ident().clone();
        if return_ty == &Type::Unit || always_returns(&body) {
            Ok(Fn::new(node.id(), ident, params, *return_ty, body))
        } else {
            Err(Error::MissingReturn(ident))
        }
    }

    fn check_static(&self, node: &untyped::Static) -> Result<Static> {
        let Symbol::Global(ty) = self.bindings.get(node)
        else { unreachable!() };

        let init = self.check_expr(node.init())?;
        ty::expect(*ty, ty::of(&init))?;

        Ok(Static::new(node.id(), node.ident().clone(), *ty, init))
    }
}

fn always_returns(block: &Block) -> bool {
    block.iter().any(|stmt| match stmt {
        Stmt::Return(_) => true,
        Stmt::If(node) => {
            always_returns(node.then_body()) &&
                node.else_body()
                    .as_ref()
                    .map(always_returns)
                    .unwrap_or(false)
        },
        _ => false,
    })
}
