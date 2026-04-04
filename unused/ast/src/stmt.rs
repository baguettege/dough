use crate::Expr;
use crate::types;

crate::macros::ast! {
    Stmt {
        ExprStmt {
            expr: Expr,
        },
        Block {
            stmts: Vec<Stmt>,
        },
        Local {
            ident: types::Ident,
            ty: types::Type,
            init: Expr,
        },
        Assign {
            target: Expr,
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
