use crate::heap::handle::Handle;

#[derive(Debug)]
pub struct Array {
    elements: Vec<Handle>
}

impl Array {
    pub fn new() -> Self {
        Self {
            elements: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub(crate) fn references(&self) -> &[Handle] {
        &self.elements
    }

    pub fn set(&mut self, index: usize, handle: Handle) {
        self.elements[index] = handle;
    }

    pub fn get(&self, index: usize) -> &Handle {
        &self.elements[index]
    }

    pub fn push(&mut self, handle: Handle) {
        self.elements.push(handle);
    }
}