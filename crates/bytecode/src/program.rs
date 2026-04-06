use crate::Chunk;

pub struct Program {
    main: Chunk,
    functions: Vec<Chunk>,
}

impl Program {
    pub fn new(main: Chunk, functions: Vec<Chunk>) -> Self {
        Self { main, functions }
    }
    
    pub fn main(&self) -> &Chunk {
        &self.main
    }
    
    pub fn functions(&self) -> &[Chunk] {
        &self.functions
    }
}
