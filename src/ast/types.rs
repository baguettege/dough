use crate::span::{Span, Spanned};

pub(crate) type TypeRef = Name;

pub(crate) struct Name {
    name: String,
    span: Span,
}

impl Name {
    pub(crate) fn new(name: impl Into<String>, span: Span) -> Self {
        Self { name: name.into(), span }
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.name
    }
}

impl Spanned for Name {
    fn span(&self) -> Span {
        self.span
    }
}

pub(crate) struct Param {
    name: Name,
    type_ref: TypeRef,
    span: Span,
}

impl Param {
    pub(crate) fn new(name: Name, type_ref: TypeRef, span: Span) -> Self {
        Self { name, type_ref, span }
    }

    pub(crate) fn name(&self) -> &Name {
        &self.name
    }

    pub(crate) fn type_ref(&self) -> &TypeRef {
        &self.type_ref
    }
}

impl Spanned for Param {
    fn span(&self) -> Span {
        self.span
    }
}

pub(crate) enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

pub(crate) enum BinaryOp {
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

pub(crate) enum UnaryOp {
    Not,
    Neg,
}
