use ast::typed::Expr;
use ast::types::Literal;
use dough_core::Type;
use crate::{Error, Result};

pub(super) fn of(expr: &Expr) -> Type {
    match expr {
        Expr::LiteralExpr(lit) => match lit.literal() {
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::Bool(_) => Type::Bool,
            Literal::Str(_) => Type::Str,
        },
        Expr::Ident(node) => *node.ty(),
        Expr::Binary(node) => *node.ty(),
        Expr::Unary(node) => *node.ty(),
        Expr::Call(node) => *node.ty(),
    }
}

pub(super) fn expect(expected: Type, found: Type) -> Result<()> {
    if expected == found {
        Ok(())
    } else {
        Err(Error::TypeMismatch { expected, found })
    }
}
