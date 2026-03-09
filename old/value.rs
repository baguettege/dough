use crate::value::array::Array;
use crate::value::function::Function;

pub mod array;
pub mod function;

pub enum DoughObject {
    Nil,
    Number(f64),
    Bool(bool),
    String(String),
    Array(Array),
    Function(Function)
}