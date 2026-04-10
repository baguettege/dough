use crate::Chunk;

pub struct Program {
    entry: Chunk,
    funcs: Vec<Chunk>,
}

impl Program {
    pub fn new(entry: Chunk, funcs: Vec<Chunk>) -> Self {
        Self { entry, funcs }
    }

    pub fn entry(&self) -> &Chunk {
        &self.entry
    }

    pub fn funcs(&self) -> &[Chunk] {
        &self.funcs
    }
}
