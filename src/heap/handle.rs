#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Handle {
    index: usize,
    generation: u32
}

impl Handle {
    pub(in crate::heap) fn new(index: usize, generation: u32) -> Self {
        Self { index, generation }
    }

    pub(in crate::heap) fn index(&self) -> usize {
        self.index
    }

    pub(in crate::heap) fn generation(&self) -> u32 {
        self.generation
    }
}