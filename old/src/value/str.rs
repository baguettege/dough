use crate::heap::{HeapObject, Trace, Tracer};

/// A string value allocated on the heap.
pub(crate) struct DoughStr {
    string: String,
}

impl DoughStr {
    pub(crate) fn new(string: impl Into<String>) -> Self {
        Self { string: string.into() }
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.string
    }

    pub(crate) fn len(&self) -> usize {
        self.string.len()
    }
}

impl Trace for DoughStr {
    fn trace(&self, _: &mut Tracer) {}
}

impl HeapObject for DoughStr {}
