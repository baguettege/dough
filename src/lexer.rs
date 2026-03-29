use crate::lexer::token::{Token, TokenKind};
use crate::span::Span;

mod token;

pub(crate) enum Error {
    UnexpectedChar(char, Span),
    UnterminatedString(Span),
    InvalidNumber(String, Span),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

/// A cursor over source text.
struct Cursor<'a> {
    source: &'a str,
    pos: usize,
    // cache the next char when peeking
    next: Option<char>,
}

impl<'a> Cursor<'a> {
    /// Sentinel to signal the end of the source, used to prevent
    /// `Option<char>` from propagating everywhere.
    const EOF_CHAR: char = '\0';

    fn new(source: &'a str) -> Self {
        Self { source, pos: 0, next: None }
    }

    /// Returns `true` if the cursor is at the end of the source.
    fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    /// Returns the next character without advancing.
    ///
    /// # Notes
    /// Returns '\0' if the cursor is at the end of the source.
    fn peek(&mut self) -> char {
        if let Some(c) = self.next {
            return c;
        }

        let c = self.source[self.pos..]
            .chars()
            .next()
            .unwrap_or(Self::EOF_CHAR);

        self.next = Some(c);
        c
    }

    /// Advances the cursor and returns the consumed character.
    ///
    /// # Notes
    /// Returns '\0' if the cursor is at the end of the source.
    fn advance(&mut self) -> char {
        let c = self.peek();

        if c != Self::EOF_CHAR {
            self.pos += c.len_utf8();
        }

        self.next = None;
        c
    }

    /// Advances while `pred` holds and the cursor is not at the end
    /// of the source, returning the consumed slice.
    fn advance_while(&mut self, pred: impl Fn(char) -> bool) -> &str {
        let start = self.pos;

        while !self.is_at_end() && pred(self.peek()) {
            self.advance();
        }

        &self.source[start..self.pos]
    }

    /// Advances and returns `true` if the next character equals `c`.
    fn match_char(&mut self, c: char) -> bool {
        if self.peek() == c {
            self.advance();
            true
        } else {
            false
        }
    }
}

/// A lexer for Dough source text.
pub(crate) struct Lexer<'a> {
    cursor: Cursor<'a>,
    start: usize,
}

impl Lexer<'_> {
    fn new_span(&self) -> Span {
        Span::new(self.start, self.cursor.pos)
    }

    fn new_token(&self, kind: TokenKind) -> Token {
        Token::new(kind, self.new_span())
    }

    /// Advances the cursor and returns `kind`.
    fn single(&mut self, kind: TokenKind) -> TokenKind {
        self.cursor.advance();
        kind
    }

    /// Advances the cursor and returns `two`, advancing the cursor
    /// once again, if the next character equals `c`, otherwise `one`.
    fn double(&mut self, c: char, one: TokenKind, two: TokenKind) -> TokenKind {
        self.cursor.advance();
        if self.cursor.match_char(c) {
            two
        } else {
            one
        }
    }
}

impl<'a> Lexer<'a> {
    /// Tokenizes `source`, returning a list of tokens or [`Error`] on failure.
    pub(crate) fn tokenize(source: &'a str) -> Result<Vec<Token>> {
        Self::new(source).tokenize_inner()
    }

    fn new(source: &'a str) -> Self {
        Self { cursor: Cursor::new(source), start: 0 }
    }

    fn tokenize_inner(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.cursor.is_at_end() {
            self.cursor.advance_while(|c| c.is_whitespace());
            self.start = self.cursor.pos;

            if self.cursor.is_at_end() {
                break;
            }

            tokens.push(self.scan_token()?);
        }

        tokens.push(self.new_token(TokenKind::Eof));
        Ok(tokens)
    }

    /// Scans the next token from the current position.
    ///
    /// # Panics
    /// If the cursor is at the end of the source.
    fn scan_token(&mut self) -> Result<Token> {
        assert!(!self.cursor.is_at_end());

        let kind = match self.cursor.peek() {
            // punctuation
            '(' => self.single(TokenKind::LParen),
            ')' => self.single(TokenKind::RParen),
            '{' => self.single(TokenKind::LBrace),
            '}' => self.single(TokenKind::RBrace),
            '[' => self.single(TokenKind::LBrack),
            ']' => self.single(TokenKind::RBrack),
            ':' => self.single(TokenKind::Colon),
            ';' => self.single(TokenKind::Semicolon),
            ',' => self.single(TokenKind::Comma),
            '.' => self.single(TokenKind::Dot),

            // 1 char operators
            '+' => self.single(TokenKind::Plus),
            '-' => self.single(TokenKind::Minus),
            '*' => self.single(TokenKind::Star),
            '/' => self.single(TokenKind::Slash),

            // 1-2 char operators
            '=' => self.double('=', TokenKind::Eq, TokenKind::EqEq),
            '<' => self.double('=', TokenKind::Lt, TokenKind::Le),
            '>' => self.double('=', TokenKind::Gt, TokenKind::Ge),
            '!' => {
                // essentially the same as self.double but `!` does not exist
                self.cursor.advance();
                if self.cursor.match_char('=') {
                    TokenKind::Ne
                } else {
                    return Err(Error::UnexpectedChar('!', self.new_span()))
                }
            },

            // other
            c if c.is_ascii_digit() => self.scan_number()?,
            // only allow alphabet/_ on 1st ident char
            c if c.is_alphabetic() || c == '_' => self.scan_ident()?,
            '"' => self.scan_str()?,

            c => return Err(Error::UnexpectedChar(c, self.new_span()))
        };

        Ok(self.new_token(kind))
    }

    /// Scans a number literal.
    ///
    /// # Panics
    /// If the current character is not an ascii digit.
    fn scan_number(&mut self) -> Result<TokenKind> {
        assert!(self.cursor.peek().is_ascii_digit());

        let s = self.cursor.advance_while(
            |c| c.is_ascii_digit() || c == '.')
            .to_string();

        // validate after for simplicity as it is a micro-optimization
        let dot_count = s.chars()
            .filter(|&c| c == '.')
            .count();

        let span = self.new_span();

        match dot_count {
            0 => s.parse::<i64>()
                .map(TokenKind::Int)
                .map_err(|_| Error::InvalidNumber(s, span)),

            1 => s.parse::<f64>()
                .map(TokenKind::Float)
                .map_err(|_| Error::InvalidNumber(s, span)),

            _ => Err(Error::InvalidNumber(s, span))
        }
    }

    /// Scans an identifier or keyword.
    ///
    /// # Panics
    /// If the current character is not `_` or alphanumeric.
    fn scan_ident(&mut self) -> Result<TokenKind> {
        let c = self.cursor.peek();
        assert!(c.is_alphanumeric() || c == '_');

        let s = self.cursor.advance_while(
            |c| c.is_alphanumeric() || c == '_')
            .to_string();

        let kind = match s.as_str() {
            "true" => TokenKind::Bool(true),
            "false" => TokenKind::Bool(false),
            "var" => TokenKind::Var,
            "func" => TokenKind::Func,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "not" => TokenKind::Not,
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            _ => TokenKind::Ident(s)
        };

        Ok(kind)
    }

    /// Scans a string literal.
    ///
    /// # Panics
    /// If the current character is not `"`.
    fn scan_str(&mut self) -> Result<TokenKind> {
        assert_eq!(self.cursor.advance(), '"');

        let s = self.cursor.advance_while(
            |c| c != '"').to_string();

        if self.cursor.match_char('"') {
            Ok(TokenKind::Str(s))
        } else {
            Err(Error::UnterminatedString(self.new_span()))
        }
    }
}
