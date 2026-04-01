use crate::span::{Span, Spanned};

#[derive(Debug)]
pub(crate) struct TypeRef {
    kind: TypeKind,
    span: Span,
}

impl TypeRef {
    pub(crate) fn new(kind: TypeKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub(crate) fn kind(&self) -> &TypeKind {
        &self.kind
    }
}

impl Spanned for TypeRef {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug)]
pub(crate) enum TypeKind {
    Name(Name),
    Array(Box<TypeRef>),
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub(crate) enum UnaryOp {
    Not,
    Neg,
}
