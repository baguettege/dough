use crate::bytecode::constant::Constant;

pub(crate) struct Chunk {
    code: Vec<u8>,
    constants: Vec<Constant>
}

impl Chunk {
    pub(crate) fn new(code: Vec<u8>, constants: Vec<Constant>) -> Self {
        Self { code, constants }
    }

    pub(crate) fn read_u8(&self, offset: usize) -> u8 {
        self.code[offset]
    }

    pub(crate) fn get_constant(&self, index: u16) -> &Constant {
        &self.constants[index as usize]
    }
}