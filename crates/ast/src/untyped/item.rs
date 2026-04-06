use crate::types::TypeRef;
use crate::untyped::stmt::Block;
use crate::untyped::Expr;

node! {
    Item {
        Fn {
            ident: String,
            params: Vec<Param>,
            return_ty: Option<TypeRef>,
            body: Block,
        },
        Static {
            ident: String,
            ty: TypeRef,
            init: Expr,
        },
    }
}

#[derive(Debug)]
pub struct Param {
    ident: String,
    ty: TypeRef,
}

impl Param {
    pub fn new(ident: String, ty: TypeRef) -> Self {
        Self { ident, ty }
    }

    pub fn ident(&self) -> &str {
        &self.ident
    }

    pub fn ty(&self) -> &TypeRef {
        &self.ty
    }
}
