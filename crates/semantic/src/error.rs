use ast::types::{BinOp, Ident, TypeRef, UnOp};
use dough_core::Type;

#[derive(Debug)]
pub enum Error {
    UnknownType(TypeRef),
    UndefinedIdent(Ident),

    MissingMain,
    InvalidMain,

    TypeMismatch { expected: Type, found: Type },
    ArityMismatch { expected: usize, found: usize },

    InvalidBinOp { op: BinOp, ty: Type },
    InvalidUnOp { op: UnOp, ty: Type },
    NotCallable(Ident),
}

pub type Result<T> = std::result::Result<T, Error>;
