mod token;
mod error;
mod cursor;
mod lexer;
mod scan;

pub use token::Token;
pub use error::{Error, Result};

pub fn lex(input: &str) -> Result<Vec<Token>> {
    lexer::Lexer::new(input).lex()
}
