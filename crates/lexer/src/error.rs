#[derive(Debug)]
pub enum Error {
    UnexpectedEof,
    UnexpectedChar(char),
    InvalidNumber(String),
}

pub type Result<T> = std::result::Result<T, Error>;
