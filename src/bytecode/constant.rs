#[derive(Debug)]
pub(crate) enum Constant {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String)
}