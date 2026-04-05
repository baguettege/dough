//! Tokenizes source code into a list of tokens.

mod token;
mod error;
mod cursor;
mod lexer;
mod scan;

pub use token::Token;
pub use error::{Error, Result};

/// Lexes `input` into a list of [`Token`]s, terminated by [`Token::Eof`].
pub fn lex(input: &str) -> Result<Vec<Token>> {
    lexer::Lexer::new(input).lex()
}
