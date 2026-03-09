use crate::bytecode::stream::BytecodeStream;

pub(crate) mod stream;
pub(crate) mod chunk;
pub(crate) mod proto_constant;

pub(crate) struct Bytecode {
    bytes: Vec<u8>
}

impl Bytecode {
    pub(crate) fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub(crate) fn len(&self) -> usize {
        self.bytes.len()
    }

    pub(crate) fn read_u8(&self, offset: usize) -> u8 {
        // SAFETY: byte exists at offset
        self.bytes.get(offset).copied().expect("Offset out of bounds")
    }

    pub(crate) fn read_u16(&self, offset: usize) -> u16 {
        // SAFETY: byte exists at offset -> offset+1
        let hi = self.read_u8(offset) as u16;
        let lo = self.read_u8(offset + 1) as u16;
        (hi << 8) | lo
    }

    pub(crate) fn stream(&self) -> BytecodeStream<'_> {
        BytecodeStream::new(self)
    }
}