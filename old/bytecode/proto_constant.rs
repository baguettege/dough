use crate::bytecode::chunk::Chunk;

pub(crate) enum ProtoConstant {
    Number(f64),
    Bool(bool),
    String(String),
    Function(Chunk)
}