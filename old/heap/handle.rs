#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Handle {
    pub(in crate::heap) index: usize,
    pub(in crate::heap) generation: u32
}

impl Handle {
    pub(in crate::heap) fn new(index: usize, generation: u32) -> Self {
        Self {
            index,
            generation
        }
    }
}