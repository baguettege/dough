use crate::types::{BinOp, Ident, Literal, UnOp};

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Ident(Ident),

    Binary {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
    Unary {
        op: UnOp,
        expr: Box<Expr>,
    },
    Call {
        callee: Ident,
        args: Vec<Expr>,
    },
}
