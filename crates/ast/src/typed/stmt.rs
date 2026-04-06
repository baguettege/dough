use crate::typed::Expr;
use dough_core::Type;

node! {
    Stmt {
        ExprStmt {
            expr: Expr,
        },
        Let {
            ident: String,
            ty: Type,
            init: Expr,
        },
        Assign {
            target: String,
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
}

pub type Block = Vec<Stmt>;
