use ast::Program;
use lexer::Token;
use crate::cursor::Cursor;
use crate::{Result, Error};

macro_rules! expect {
    ($self:ident, $token:pat) => {
        match $self.cursor.advance() {
            Some($token) => Ok(()),
            Some(token) => Err(Error::UnexpectedToken(token.clone())),
            None => Err(Error::UnexpectedEof),
        }?;
    };
    
    ($self:ident, $( $token:pat ),* $(,)?) => {
        $( expect!($self, $token); )*
    }
}

mod expr;
mod stmt;
mod item;
mod common;

pub(crate) struct Parser<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens: &'a [Token]) -> Self {
        Self { cursor: Cursor::new(tokens) }
    }
    
    pub(crate) fn parse(mut self) -> Result<Program> {
        let mut items = Vec::new();
        
        while !self.cursor.is_at_end() {
            let item = self.parse_item()?;
            items.push(item);
        }
        
        Ok(items)
    }
}
