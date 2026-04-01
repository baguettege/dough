use crate::heap::{Handle, Trace, Tracer};
use crate::value::array::DoughArray;
use crate::value::str::DoughStr;

mod array;
mod str;

/// A runtime value in the Dough VM.
pub(crate) enum DoughValue {
    /// The unit value, used as a sentinel for uninitialized slots
    /// and void function returns. Cannot be produced by user code.
    Unit,
    
    Int(i64),
    Float(f64),
    Bool(bool),

    Str(Handle<DoughStr>),
    Array(Handle<DoughArray>),
}

macro_rules! primitive_accessors {
    ($( $variant:ident, $type:ty; )+) => {
        paste::paste! {
            $(
                pub(crate) fn [<as_ $variant:lower>](&self) -> $type {
                    if let Self::$variant(v) = self {
                        *v
                    } else {
                        unreachable!("expected {}", stringify!([<$variant:lower>]));
                    }
                }
            )+
        }
    };
}

macro_rules! handle_accessors {
    ($( $variant: ident, $type:ty; )+) => {
        paste::paste! {
            $(
                pub(crate) fn [<as_ $variant:lower _handle>](&self) -> &Handle<$type> {
                    if let Self::$variant(v) = self {
                        v
                    } else {
                        unreachable!("expected {} handle", stringify!([<$variant:lower>]));
                    }
                }

                pub(crate) fn [<as_ $variant:lower _handle_mut>](&mut self) -> &mut Handle<$type> {
                    if let Self::$variant(v) = self {
                        v
                    } else {
                        unreachable!("expected {} handle", stringify!([<$variant:lower>]));
                    }
                }
            )+
        }
    };
}

impl DoughValue {
    primitive_accessors! {
        Int, i64;
        Float, f64;
        Bool, bool;
    }

    handle_accessors! {
        Str, DoughStr;
        Array, DoughArray;
    }
}

impl Trace for DoughValue {
    fn trace(&self, tracer: &mut Tracer) {
        match self {
            DoughValue::Str(handle) => tracer.push(handle),
            DoughValue::Array(handle) => tracer.push(handle),
            _ => {}
        }
    }
}
