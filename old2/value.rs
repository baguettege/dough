use crate::heap::handle::Handle;
use crate::value::array::DoughArray;
use crate::value::closure::DoughClosure;
use crate::value::str::DoughStr;
use crate::value::upvalue::DoughUpvalue;

pub mod array;
pub mod str;
mod upvalue;
mod closure;

pub enum DoughValue {
    Unit,
    Int(i64),
    Float(f64),
    Bool(bool),
    Object(Handle)
}

pub(crate) enum DoughObject {
    Str(DoughStr),
    Array(DoughArray),
    Closure(DoughClosure),
    Upvalue(DoughUpvalue)
}