use crate::untyped::Expr;
use crate::untyped::stmt::Block;
use crate::types::{Ident, TypeRef};

#[derive(Debug)]
pub enum Item {
    Fn {
        ident: Ident,
        params: Vec<Param>,
        return_ty: Option<TypeRef>,
        body: Block,
    },
    Static {
        ident: Ident,
        ty: TypeRef,
        init: Expr,
    },
}

#[derive(Debug)]
pub struct Param {
    ident: Ident,
    ty: TypeRef,
}

impl Param {
    pub fn new(ident: Ident, ty: TypeRef) -> Self {
        Self { ident, ty }
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn ty(&self) -> &TypeRef {
        &self.ty
    }
}
