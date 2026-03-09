use crate::heap::handle::Handle;
use crate::heap::trace::GcTrace;
use crate::value::array::DoughArray;
use crate::value::str::DoughStr;

pub mod str;
pub mod array;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DoughValue {
    Unit,
    Int(i64),
    Float(f64),
    Bool(bool),
    Object(Handle)
}

impl DoughValue {
    pub fn as_i64(&self) -> i64 {
        match self {
            DoughValue::Int(i) => *i,
            _ => panic!("expected int, got {:?}", self)
        }
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            DoughValue::Float(f) => *f,
            _ => panic!("expected float, got {:?}", self)
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            DoughValue::Bool(b) => *b,
            _ => panic!("expected bool, got {:?}", self)
        }
    }

    pub fn as_handle(&self) -> Handle {
        match self {
            DoughValue::Object(h) => *h,
            _ => panic!("expected handle, got {:?}", self)
        }
    }
}

#[derive(Debug)]
pub(crate) enum DoughObject {
    Str(DoughStr),
    Array(DoughArray)
}

impl DoughObject {
    pub(crate) fn as_str(&self) -> &DoughStr {
        match self {
            DoughObject::Str(s) => s,
            _ => panic!("expected str, got {:?}", self)
        }
    }

    pub(crate) fn as_array(&self) -> &DoughArray {
        match self {
            DoughObject::Array(array) => array,
            _ => panic!("expected array, got {:?}", self)
        }
    }

    pub(crate) fn as_array_mut(&mut self) -> &mut DoughArray {
        match self {
            DoughObject::Array(array) => array,
            _ => panic!("expected array, got {:?}", self)
        }
    }
}

impl GcTrace for DoughObject {
    fn references(&self) -> Vec<Handle> {
        match self {
            DoughObject::Str(s) => s.references(),
            DoughObject::Array(array) => array.references(),
        }
    }
}