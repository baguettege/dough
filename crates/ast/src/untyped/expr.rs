use crate::types::{BinOp, Literal, UnOp};

node! {
    Expr {
        LiteralExpr {
            literal: Literal,
        },
        Ident {
            ident: String,
        },
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
            callee: String,
            args: Vec<Expr>,
        },
    }
}
