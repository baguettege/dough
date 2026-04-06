use ast::types::{BinOp, TypeRef, UnOp};
use dough_core::Type;

#[derive(Debug)]
pub enum Error {
    UnknownType(TypeRef),
    UndefinedIdent(String),
    AlreadyDefined(String),

    NotAssignable(String),
    NotAValue(String),
    NotCallable(String),

    TypeMismatch { expected: Type, found: Type },
    ArityMismatch { expected: usize, found: usize },

    InvalidBinOp { op: BinOp, ty: Type },
    InvalidUnOp { op: UnOp, ty: Type },

    MissingMain,
    MissingReturn(String),
}

pub type Result<T> = std::result::Result<T, Error>;
