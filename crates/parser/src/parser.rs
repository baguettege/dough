use ast::NodeId;
use crate::cursor::Cursor;
use crate::Result;
use ast::untyped::Program;
use lexer::Token;

macro_rules! expect {
    ($self:ident, $token:pat) => {
        match $self.cursor.advance() {
            Some($token) => Ok(()),
            Some(token) => Err(Error::UnexpectedToken(token.clone())),
            None => Err(Error::UnexpectedEof),
        }?;
    };
}

mod expr;
mod stmt;
mod item;
mod common;

pub(crate) struct Parser<'a> {
    cursor: Cursor<'a>,
    id: IdGen,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens: &'a [Token]) -> Self {
        let (cursor, id) = (Cursor::new(tokens), IdGen::new());
        Self { cursor, id }
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

struct IdGen(NodeId);

impl IdGen {
    fn new() -> Self {
        Self(0)
    }

    fn next(&mut self) -> NodeId {
        let id = self.0;
        self.0 += 1;
        id
    }
}
