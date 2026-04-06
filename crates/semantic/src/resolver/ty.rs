use ast::types::TypeRef;
use dough_core::Type;
use crate::{Result, Error};

pub(super) fn resolve(ty: &TypeRef) -> Result<Type> {
    match ty.as_str() {
        "int" => Ok(Type::Int),
        "float" => Ok(Type::Float),
        "bool" => Ok(Type::Bool),
        "str" => Ok(Type::Str),
        _ => Err(Error::UnknownType(ty.clone()))
    }
}
