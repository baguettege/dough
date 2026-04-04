use crate::{Result, Error};

pub(crate) struct Cursor<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    pub(crate) fn peek(&mut self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    pub(crate) fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            self.pos += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    pub(crate) fn expect(&mut self, c: char) -> Result<()> {
        match self.advance() {
            Some(ch) if ch == c => Ok(()),
            Some(ch) => Err(Error::UnexpectedChar(ch)),
            None => Err(Error::UnexpectedEof),
        }
    }

    pub(crate) fn advance_while(&mut self, pred: impl Fn(char) -> bool) -> &str {
        let start_pos = self.pos;

        while let Some(c) = self.peek() {
            if !pred(c) {
                break;
            }
            
            self.advance();
        }

        &self.input[start_pos..self.pos]
    }

    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn is_at_end(&mut self) -> bool {
        self.peek().is_none()
    }
}
