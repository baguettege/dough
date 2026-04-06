use dough_core::Type;

#[derive(Debug, Clone)]
pub enum Symbol {
    Global(Type),
    Local(Type),

    Fn {
        params: Vec<Type>,
        return_ty: Type,
    },
}
