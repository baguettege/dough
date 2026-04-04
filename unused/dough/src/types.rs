pub enum Type {
    Int,
    Float,
    Bool,
    Str,
    Array(Box<Type>),
}
