use crate::heap::handle::Handle;
use crate::heap::trace::GcTrace;
use crate::value::DoughValue;

#[derive(Debug)]
pub struct DoughArray(Vec<DoughValue>);

impl DoughArray {
    pub fn new(values: Vec<DoughValue>) -> Self {
        Self(values)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> DoughValue {
        self.0[index]
    }

    pub fn set(&mut self, index: usize, value: DoughValue) {
        self.0[index] = value;
    }

    pub(crate) fn push(&mut self, value: DoughValue) {
        self.0.push(value)
    }
}

impl GcTrace for DoughArray {
    fn references(&self) -> Vec<Handle> {
        self.0.iter()
            .filter_map(|val| match val {
                DoughValue::Object(handle) => Some(*handle),
                _ => None
            }).collect()
    }
}