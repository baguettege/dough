use crate::Opcode;

#[derive(Debug)]
pub enum Error {
    UnexpectedEof,
    UnknownOpcode(Opcode),
}

pub type Result<T> = std::result::Result<T, Error>;
