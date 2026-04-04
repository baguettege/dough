use crate::Expr;
use crate::types::{Ident, TypeRef};

#[derive(Debug)]
pub enum Stmt<T = ()> {
    Expr(Expr<T>),

    Let {
        ident: Ident,
        ty: TypeRef,
        init: Expr<T>,
    },
    Assign {
        target: Ident,
        value: Expr<T>,
    },
    If {
        condition: Expr<T>,
        then_body: Block<T>,
        else_body: Option<Block<T>>,
    },
    While {
        condition: Expr<T>,
        body: Block<T>,
    },
    Return {
        value: Option<Expr<T>>,
    },
}

pub type Block<T = ()> = Vec<Stmt<T>>;
