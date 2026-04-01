#[derive(Debug)]
pub enum Type {
    Named(Ident),
    Array(Box<Type>),
}

#[derive(Debug)]
pub struct Ident(String);

impl Ident {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl std::ops::Deref for Ident {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct Param {
    ident: Ident,
    ty: Type,
}

impl Param {
    pub fn new(ident: Ident, ty: Type) -> Self {
        Self { ident, ty }
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

#[derive(Debug, Copy, Clone)]
pub enum BinOp {
    Pow,

    Mul,
    Div,
    Mod,
    FDiv,

    Add,
    Sub,

    Lt,
    Le,
    Gt,
    Ge,

    Eq,
    Ne,

    And,

    Or,
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOp {
    Not,
    Neg,
}
