use crate::heap::handle::Handle;

#[macro_export]
macro_rules! dough_arr {
    ( $( $elem:expr ),* $(,)? ) => {
        {
            let mut elems = Vec::new();
            $( elems.push($elem) )*;
            DoughArray::new(&elems)
        }
    };
}

pub struct DoughArray(Vec<Handle>);

impl DoughArray {
    pub fn new(elements: &[Handle]) -> Self {
        Self(elements.to_vec())
    }

    pub fn references(&self) -> &[Handle] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Handle {
        self.0[index]
    }

    pub fn set(&mut self, index: usize, handle: Handle) {
        self.0[index] = handle;
    }
}