#[derive(Debug)]
pub enum Error {
    OutOfRegisters,
    OutOfIndices,
    JumpOutOfRange,
}

pub type Result<T> = std::result::Result<T, Error>;
