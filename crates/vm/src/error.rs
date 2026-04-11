use dough_core::Type;

#[derive(Debug)]
pub enum Error {
    UnexpectedEof,
    IpOverflow,
    JumpOutOfRange,
    IndexOutOfBounds,

    TypeMismatch { expected: Type, found: Type },
    StackUnderflow,
    
    DivisionByZero,
}

pub type Result<T> = std::result::Result<T, Error>;
