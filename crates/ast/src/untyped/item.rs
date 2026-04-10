use crate::{Node, NodeId};
use crate::types::TypeRef;
use crate::untyped::stmt::Block;

node! {
    Item {
        Func {
            ident: String,
            params: Vec<Param>,
            return_ty: Option<TypeRef>,
            body: Block,
        },
    }
}

#[derive(Debug)]
pub struct Param {
    id: NodeId,
    ident: String,
    ty: TypeRef,
}

impl Param {
    pub fn new(id: NodeId, ident: impl Into<String>, ty: TypeRef) -> Self {
        let ident = ident.into();
        Self { id, ident, ty }
    }

    pub fn ident(&self) -> &str {
        &self.ident
    }

    pub fn ty(&self) -> &TypeRef {
        &self.ty
    }
}

impl Node for Param {
    fn id(&self) -> NodeId {
        self.id
    }
}
