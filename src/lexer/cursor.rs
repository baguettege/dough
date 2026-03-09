pub(super) struct Cursor {
    chars: Vec<char>,
    pos: usize
}

impl Cursor {
    const EOF: char = '\0';

    pub(super) fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0
        }
    }

    pub(super) fn is_at_end(&self) -> bool {
        self.pos >= self.chars.len()
    }

    pub(super) fn pos(&self) -> usize {
        self.pos
    }

    fn peek_offset(&self, offset: usize) -> char {
        match self.chars.get(self.pos + offset) {
            None => Self::EOF,
            Some(c) => *c
        }
    }

    pub(super) fn peek(&self) -> char {
        self.peek_offset(0)
    }

    pub(super) fn peek_next(&self) -> char {
        self.peek_offset(1)
    }

    pub(super) fn advance(&mut self) -> char {
        let c = self.peek();
        self.pos += 1;
        c
    }

    pub(super) fn skip(&mut self, count: usize) {
        for i in 0..count {
            self.advance();
        }
    }

    pub(super) fn check(&self, c: char) -> bool {
        self.peek() == c
    }

    pub(super) fn check_next(&self, c: char) -> bool {
        self.peek_next() == c
    }

    pub(super) fn check_two(&self, c1: char, c2: char) -> bool {
        self.check(c1) && self.check_next(c2)
    }

    pub(super) fn check_if(&self, f: impl Fn(char) -> bool) -> bool {
        f(self.peek())
    }

    pub(super) fn advance_if(&mut self, c: char) -> bool {
        let checked = self.check(c);
        if checked {
            self.advance();
        }
        checked
    }

    pub(super) fn advance_while(&mut self, f: impl Fn(char) -> bool) -> String {
        let mut chars = Vec::new();
        while f(self.peek()) && !self.is_at_end() {
            chars.push(self.advance());
        }
        chars.into_iter().collect()
    }
}