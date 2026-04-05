use dough_core::Type;
use crate::typed::{Block, Expr};
use crate::types::Ident;

#[derive(Debug)]
pub enum Item {
    Fn {
        ident: Ident,
        params: Vec<Param>,
        return_ty: Type,
        body: Block,
    },
    Static {
        ident: Ident,
        ty: Type,
        init: Expr,
    },
}

#[derive(Debug)]
pub struct Param {
    ident: Ident,
    ty: Type,
}

impl Param {
    pub fn new(ident: Ident, ty: Type) -> Self {
        Self { ident, ty }
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn ty(&self) -> Type {
        self.ty
    }
}
