use crate::types;

crate::macros::ast! {
    Expr {
        Binary {
            lhs: Box<Expr>,
            op: types::BinOp,
            rhs: Box<Expr>,
        },
        Unary {
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
}
