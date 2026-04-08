use ast::NodeId;
use dough_core::Type;

#[derive(Debug, Clone)]
pub enum Symbol {
    Global {
        ty: Type,
        id: NodeId,
    },
    Local {
        ty: Type,
        id: NodeId,
    },
    Fn {
        params: Vec<Type>,
        return_ty: Type,
        id: NodeId,
    },
}
