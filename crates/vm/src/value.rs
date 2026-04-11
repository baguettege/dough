mod str;

pub(crate) use str::Str;

use dough_core::Type;
use heap::{Handle};

#[derive(Copy, Clone)]
pub(crate) enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Unit,
    Str(Handle<Str>),
}

macro_rules! impl_try_from_value {
    ($variant:ident($ty:ty)) => {
        impl TryFrom<Value> for $ty {
            type Error = $crate::Error;

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                match value {
                    Value::$variant(v) => Ok(v),
                    _ => Err($crate::Error::TypeMismatch {
                        expected: Type::$variant,
                        found: value.ty(),
                    })
                }
            }
        }
    };
}

impl Value {
    fn ty(&self) -> Type {
        match self {
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::Bool(_) => Type::Bool,
            Value::Unit => Type::Unit,
            Value::Str(_) => Type::Str,
        }
    }
}

impl_try_from_value!(Int(i64));
impl_try_from_value!(Float(f64));
impl_try_from_value!(Bool(bool));
impl_try_from_value!(Str(Handle<Str>));
