use crate::Expr;
use crate::stmt::Block;
use crate::types::{Ident, Param, TypeRef};

#[derive(Debug)]
pub enum Item<T = ()> {
    Fn {
        ident: Ident,
        params: Vec<Param>,
        return_ty: Option<TypeRef>,
        body: Block<T>,
    },
    Global {
        ident: Ident,
        ty: TypeRef,
        init: Expr<T>,
    },
}
