use crate::types;

#[ast_macros::ast]
enum Expr {
    Binary {
        lhs: Box<Expr>,
        #[copy]
        op: types::BinOp,
        rhs: Box<Expr>,
    },
    Unary {
        #[copy]
        op: types::UnaryOp,
        expr: Box<Expr>,
    },
    Ident {
        ident: types::Ident,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    Member {
        target: Box<Expr>,
        member: types::Ident,
    },
    Literal {
        value: types::Literal,
    },
}