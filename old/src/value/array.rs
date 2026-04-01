use crate::heap::{HeapObject, Trace, Tracer};
use crate::value::DoughValue;

/// An array value allocated on the heap.
pub(crate) struct DoughArray {
    elements: Vec<DoughValue>,
}

impl DoughArray {
    pub(crate) fn new(len: usize) -> Self {
        let mut elements = Vec::with_capacity(len);
        elements.resize_with(len, || DoughValue::Unit);
        Self { elements }
    }

    /// Returns a reference to the element at `index`.
    ///
    /// # Panics
    /// If `index` is out of bounds.
    pub(crate) fn get(&self, index: usize) -> &DoughValue {
        &self.elements[index]
    }

    /// Set the element at `index` to `value`.
    ///
    /// # Panics
    /// If `index` is out of bounds.
    pub(crate) fn set(&mut self, index: usize, value: DoughValue) {
        self.elements[index] = value;
    }

    pub(crate) fn len(&self) -> usize {
        self.elements.len()
    }
}

impl Trace for DoughArray {
    fn trace(&self, tracer: &mut Tracer) {
        for element in self.elements.iter() {
            element.trace(tracer);
        }
    }
}

impl HeapObject for DoughArray {}
