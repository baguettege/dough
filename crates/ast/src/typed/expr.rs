use dough_core::Type;
use crate::types::{BinOp, Literal, UnOp};

node! {
    Expr {
        LiteralExpr {
            literal: Literal,
        },
        Ident {
            ident: String,
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
            callee: String,
            args: Vec<Expr>,
            ty: Type,
        },
    }
}
