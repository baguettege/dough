use crate::typed::{Block, Expr};
use dough_core::Type;

node! {
    Item {
        Fn {
            ident: String,
            params: Vec<Param>,
            return_ty: Type,
            body: Block,
        },
        Static {
            ident: String,
            ty: Type,
            init: Expr,
        },
    }
}

#[derive(Debug)]
pub struct Param {
    ident: String,
    ty: Type,
}

impl Param {
    pub fn new(ident: String, ty: Type) -> Self {
        Self { ident, ty }
    }

    pub fn ident(&self) -> &str {
        &self.ident
    }

    pub fn ty(&self) -> Type {
        self.ty
    }
}
