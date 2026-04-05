//! Supporting types for the AST.

use std::ops::Deref;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Ident(String);

impl Ident {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl Deref for Ident {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct TypeRef(Ident);

impl TypeRef {
    pub fn new(ident: Ident) -> Self {
        Self(ident)
    }
}

impl Deref for TypeRef {
    type Target = Ident;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BinOp {
    Mul,
    Div,

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
pub enum UnOp {
    Not,
    Neg,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}
