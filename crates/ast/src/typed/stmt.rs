use dough_core::Type;
use crate::typed::Expr;
use crate::types::Ident;

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),

    Let {
        ident: Ident,
        ty: Type,
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
