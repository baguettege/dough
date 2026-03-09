use crate::bytecode::chunk::Chunk;

pub(crate) struct Proto {
    name: String,
    chunk: Chunk,
    arity: u8,
    reg_count: usize
}

impl Proto {
    pub(crate) fn new(
        name: String, chunk: Chunk,
        arity: u8, reg_count: usize
    ) -> Self {
        Self { name, chunk, arity, reg_count }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn chunk(&self) -> &Chunk {
        &self.chunk
    }

    pub(crate) fn arity(&self) -> u8 {
        self.arity
    }

    pub(crate) fn reg_count(&self) -> usize {
        self.reg_count
    }
}