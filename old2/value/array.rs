use crate::heap::handle::Handle;
use crate::value::DoughValue;

#[macro_export]
macro_rules! dough_array {
    ( $( $val:expr ),* $(,)? ) => {
        {
            let mut values = Vec::new();
            $( values.push($val) )*;
            DoughArray::new(values)
        }
    };
}

pub struct DoughArray {
    elements: Vec<DoughValue>
}

impl DoughArray {
    pub fn new(elements: Vec<DoughValue>) -> Self {
        Self { elements }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn get(&self, index: usize) -> &DoughValue {
        &self.elements[index]
    }

    pub fn set(&mut self, index: usize, value: DoughValue) {
        self.elements[index] = value;
    }

    pub(crate) fn references(&self) -> Vec<Handle> {
        self.elements.iter()
            .filter_map(|v| match v {
                DoughValue::Object(handle) => Some(*handle),
                _ => None
            }).collect()
    }
}