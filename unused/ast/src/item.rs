use crate::{types, Block, Expr};

crate::macros::ast! {
    Item {
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
}
