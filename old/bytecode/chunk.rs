use crate::bytecode::Bytecode;
use crate::bytecode::proto_constant::ProtoConstant;

pub(crate) struct Chunk {
    bytecode: Bytecode,
    constants: Vec<ProtoConstant>,
    arity: u8
}

impl Chunk {
    pub(crate) fn new(bytecode: Bytecode, constants: Vec<ProtoConstant>, arity: u8) -> Self {
        Self {
            bytecode,
            constants,
            arity
        }
    }

    pub(crate) fn unpack(self) -> (Bytecode, Vec<ProtoConstant>, u8) {
        (self.bytecode, self.constants, self.arity)
    }
}