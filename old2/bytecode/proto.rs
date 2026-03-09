use crate::bytecode::Chunk;

pub(crate) struct FunctionProto {
    name: String,
    arity: u8,
    upvalue_count: usize,
    chunk: Chunk
}