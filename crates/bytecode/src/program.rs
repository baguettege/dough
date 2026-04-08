use crate::Chunk;

pub struct Program {
    main: Chunk,
    fns: Vec<Chunk>,
    global_count: usize,
}

impl Program {
    pub fn new(main: Chunk, fns: Vec<Chunk>, global_count: usize) -> Self {
        Self { main, fns, global_count }
    }

    pub fn main(&self) -> &Chunk {
        &self.main
    }

    pub fn fns(&self) -> &[Chunk] {
        &self.fns
    }
    
    pub fn global_count(&self) -> usize {
        self.global_count
    }
}
