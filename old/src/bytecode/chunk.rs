/// A compile-time constant.
pub(crate) enum Constant {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

/// A compiled unit of bytecode, including instructions and a constant pool.
pub(crate) struct Chunk {
    code: Vec<u8>,
    constants: Vec<Constant>,
}

impl Chunk {
    pub(crate) fn new(code: Vec<u8>, constants: Vec<Constant>) -> Self {
        Self { code, constants }
    }

    pub(crate) fn code(&self) -> &[u8] {
        &self.code
    }

    /// Returns a reference to the constant at `index`.
    /// 
    /// # Panics
    /// If `index` is out of bounds.
    pub(crate) fn constant(&self, index: usize) -> &Constant {
        &self.constants[index]
    }
}

/// A compiled program, containing a main chunk and table of functions.
pub(crate) struct Program {
    main: Chunk,
    functions: Vec<Chunk>,
}

impl Program {
    pub(crate) fn new(main: Chunk, functions: Vec<Chunk>) -> Self {
        Self { main, functions }
    }

    pub(crate) fn main(&self) -> &Chunk {
        &self.main
    }

    /// Returns a reference to the chunk at `index`.
    ///
    /// # Panics
    /// If `index` is out of bounds.
    pub(crate) fn function(&self, index: usize) -> &Chunk {
        &self.functions[index]
    }
}
