use crate::symbol::Symbol;
use crate::typeck::TypeChecker;
use crate::{Error, Result};
use ast::typed::{Block, Func, Item, Param, Stmt};
use ast::{untyped, Node};

impl TypeChecker<'_> {
    pub(super) fn check_item(&mut self, item: &untyped::Item) -> Result<Item> {
        match item {
            untyped::Item::Func(node) => self.check_func(node).map(Into::into),
        }
    }

    fn check_func(&mut self, node: &untyped::Func) -> Result<Func> {
        // resolver guarantees this is a `Symbol::Func`
        let Symbol::Func { params, return_ty, .. } = self.bindings.get(node)
        else { unreachable!() };

        let params = params
            .iter()
            .zip(node.params())
            .map(|(&ty, param)|
                Param::new(param.id(), param.ident(), ty))
            .collect::<Vec<_>>();

        self.return_ty = Some(*return_ty);
        let body = self.check_block(node.body());
        self.return_ty = None;
        let body = body?;

        let ident = node.ident().clone();
        if always_returns(&body) {
            Ok(Func::new(node.id(), ident, params, *return_ty, body))
        } else {
            Err(Error::MissingReturn(ident))
        }
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
