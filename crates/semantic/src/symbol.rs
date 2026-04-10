use ast::NodeId;
use dough_core::Type;

#[derive(Debug, Clone)]
pub enum Symbol {
    Local {
        ty: Type,
        id: NodeId,
    },
    Func {
        params: Vec<Type>,
        return_ty: Type,
        id: NodeId,
    },
}
