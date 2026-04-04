use lexer::Token;

#[derive(Debug)]
pub enum Error {
    UnexpectedEof,
    UnexpectedToken(Token),
}

pub type Result<T> = std::result::Result<T, Error>;
