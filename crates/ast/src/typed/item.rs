use crate::typed::Block;
use crate::{Node, NodeId};
use dough_core::Type;

node! {
    Item {
        Func {
            ident: String,
            params: Vec<Param>,
            return_ty: Type,
            body: Block,
        },
    }
}

#[derive(Debug)]
pub struct Param {
    id: NodeId,
    ident: String,
    ty: Type,
}

impl Param {
    pub fn new(id: NodeId, ident: impl Into<String>, ty: Type) -> Self {
        let ident = ident.into();
        Self { id, ident, ty }
    }

    pub fn ident(&self) -> &str {
        &self.ident
    }

    pub fn ty(&self) -> Type {
        self.ty
    }
}

impl Node for Param {
    fn id(&self) -> NodeId {
        self.id
    }
}
