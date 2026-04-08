pub enum Constant {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Constant>,
    local_count: usize
}

impl Chunk {
    pub fn new(code: Vec<u8>, constants: Vec<Constant>, local_count: usize) -> Self {
        Self { code, constants, local_count }
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }
    
    pub fn local_count(&self) -> usize {
        self.local_count
    }
}
