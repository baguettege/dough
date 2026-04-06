pub enum Constant {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Constant>,
}

impl Chunk {
    pub fn new(code: Vec<u8>, constants: Vec<Constant>) -> Self {
        Self { code, constants }
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }
}
