use std::str::Chars;

pub(crate) struct Cursor<'a> {
    source: &'a str,
    pos: usize,
}

impl<'a> Cursor<'a> {
    const EOF: char = '\0';

    pub(crate) fn new(source: &'a str) -> Self {
        Self { source, pos: 0 }
    }

    pub(crate) fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    pub(crate) fn peek(&self) -> char {
        self.source[self.pos..]
            .chars()
            .next()
            .unwrap_or(Self::EOF)
    }

    pub(crate) fn advance(&mut self) -> Option<char> {
        todo!()
    }

    pub(crate) fn take_while(&mut self, pred: impl Fn(char) -> bool) -> &str {

        todo!()
    }
}
