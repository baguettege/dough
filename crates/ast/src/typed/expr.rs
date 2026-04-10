use dough_core::Type;
use crate::NodeId;
use crate::types::{BinOp, Literal, UnOp};

node! {
    Expr {
        LiteralExpr {
            literal: Literal,
            ty: Type,
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
            binding: NodeId,
        },
    }
}

impl Expr {
    pub fn ty(&self) -> Type {
        match self {
            Expr::LiteralExpr(node) => node.ty,
            Expr::Ident(node) => node.ty,
            Expr::Binary(node) => node.ty,
            Expr::Unary(node) => node.ty,
            Expr::Call(node) => node.ty,
        }
    }
}
