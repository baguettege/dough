use ast::typed::Expr;
use ast::types::{Literal, TypeRef};
use dough_core::Type;
use crate::{Error, Result};

pub(super) fn ty(ty: &TypeRef) -> Result<Type> {
    match ty.as_ref() {
        "int" => Ok(Type::Int),
        "float" => Ok(Type::Float),
        "bool" => Ok(Type::Bool),
        "str" => Ok(Type::Str),
        _ => Err(Error::UnknownType(ty.clone())),
    }
}

pub(super) fn expr(expr: &Expr) -> Type {
    match expr {
        Expr::Literal(lit) => match lit {
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::Bool(_) => Type::Bool,
            Literal::Str(_) => Type::Str,
        },
        Expr::Ident { ty, .. } => *ty,
        Expr::Binary { ty, .. } => *ty,
        Expr::Unary { ty, .. } => *ty,
        Expr::Call { ty, .. } => *ty,
    }
}
