use dough_core::Type;
use crate::types::{BinOp, Ident, Literal, UnOp};

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    
    Ident{
        ident: Ident,
        ty: Type,
    },
    Binary {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
        ty: Type,
    },
    Unary {
        op: UnOp,
        expr: Box<Expr>,
        ty: Type,
    },
    Call {
        callee: Ident,
        args: Vec<Expr>,
        ty: Type,
    },
}
