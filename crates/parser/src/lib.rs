mod error;
mod cursor;
mod parser;

use ast::Program;
use lexer::Token;

pub use error::{Error, Result};

pub fn parse(tokens: &[Token]) -> Result<Program> {
    parser::Parser::new(&tokens).parse()
}
