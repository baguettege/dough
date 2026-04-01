pub(crate) enum DoughType {
    Unit,
    Int,
    Float,
    Bool,
    Str,
    Array(Box<DoughType>),
}