//! Parses a token stream into an untyped AST.

mod error;
mod cursor;
mod parser;

use ast::untyped::Program;
use lexer::Token;

pub use error::{Error, Result};

/// Parses `tokens` into an untyped [`Program`].
pub fn parse(tokens: &[Token]) -> Result<Program> {
    parser::parse(tokens)
}
