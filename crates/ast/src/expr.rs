use crate::types::{BinOp, Ident, Literal, UnOp};

#[derive(Debug)]
pub enum Expr<T = ()> {
    Literal(Literal),

    Binary {
        lhs: Box<Expr<T>>,
        op: BinOp,
        rhs: Box<Expr<T>>,
        ty: T,
    },
    Unary {
        op: UnOp,
        expr: Box<Expr<T>>,
        ty: T,
    },
    Ident {
        ident: Ident,
        ty: T,
    },
    Call {
        callee: Ident,
        args: Vec<Expr<T>>,
        ty: T,
    },
}
