use crate::bytecode::Bytecode;

pub(crate) struct BytecodeStream<'a> {
    bytecode: &'a Bytecode,
    cursor: usize
}

impl<'a> BytecodeStream<'a> {
    pub(in crate::bytecode) fn new(bytecode: &'a Bytecode) -> Self {
        Self {
            bytecode,
            cursor: 0
        }
    }

    pub(crate) fn next_u8(&mut self) -> u8 {
        let b = self.bytecode.read_u8(self.cursor);
        self.cursor += 1;
        b
    }

    pub(crate) fn next_u16(&mut self) -> u16 {
        let b = self.bytecode.read_u16(self.cursor);
        self.cursor += 2;
        b
    }

    pub(crate) fn next_i8(&mut self) -> i8 {
        self.next_u8() as i8
    }

    pub(crate) fn next_i16(&mut self) -> i16 {
        self.next_u16() as i16
    }
}