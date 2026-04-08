use dough_core::Type;
use crate::NodeId;
use crate::types::{BinOp, Literal, UnOp};

node! {
    Expr {
        LiteralExpr {
            literal: Literal,
        },
        Ident {
            ident: String,
            ty: Type,
            binding: NodeId,
        },
        Binary {
            lhs: Box<Expr>,
            op: BinOp,
            rhs: Box<Expr>,
            result_ty: Type,
            operand_ty: Type,
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
            binding: NodeId,
        },
    }
}
