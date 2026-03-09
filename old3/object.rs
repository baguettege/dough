use crate::object::arr::DoughArray;
use crate::object::scalars::{DoughBool, DoughFloat, DoughInt};
use crate::object::str::DoughStr;

pub mod scalars;
pub mod str;
pub mod arr;

pub(crate) enum DoughObject {
    Nil,
    Int(DoughInt),
    Float(DoughFloat),
    Bool(DoughBool),
    Str(DoughStr),
    Array(DoughArray)
}