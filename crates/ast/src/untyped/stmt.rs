use crate::untyped::Expr;
use crate::types::{Ident, TypeRef};

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),

    Let {
        ident: Ident,
        ty: TypeRef,
        init: Expr,
    },
    Assign {
        target: Ident,
        value: Expr,
    },
    If {
        condition: Expr,
        then_body: Block,
        else_body: Option<Block>,
    },
    While {
        condition: Expr,
        body: Block,
    },
    Return {
        value: Option<Expr>,
    },
}

pub type Block = Vec<Stmt>;
