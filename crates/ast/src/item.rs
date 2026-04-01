use crate::{types, Block, Expr};

#[ast_macros::ast]
enum Item {
    Func {
        ident: types::Ident,
        params: Vec<types::Param>,
        return_ty: Option<types::Type>,
        body: Block,
    },
    Global {
        ident: types::Ident,
        ty: types::Type,
        init: Expr,
    },
}