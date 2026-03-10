use crate::error::LexError;
use crate::lexer::cursor::Cursor;
use crate::lexer::token::{Token, TokenKind};
use crate::span::Span;

pub(crate) mod token;
mod cursor;
mod token2;

pub(crate) type LexResult<T> = Result<T, LexError>;

pub(crate) struct Lexer {
    cursor: Cursor,
    start: usize
}

impl Lexer {
    pub(crate) fn lex(source: &str) -> LexResult<Vec<Token>> {
        Self::new(source).run()
    }

    fn new(source: &str) -> Self {
        Self { cursor: Cursor::new(source), start: 0 }
    }

    fn new_span(&self) -> Span {
        Span::new(self.start, self.cursor.pos())
    }

    fn new_token(&self, kind: TokenKind) -> Token {
        Token::new(kind, self.new_span())
    }

    fn run(mut self) -> LexResult<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.cursor.is_at_end() {
            let token = self.scan_token()?;
            match token.kind() {
                TokenKind::Eof => break,
                _ => tokens.push(token)
            }
        }

        tokens.push(self.new_token(TokenKind::Eof));
        Ok(tokens)
    }

    fn scan_token(&mut self) -> LexResult<Token> {
        self.cursor.advance_while(|c| c.is_whitespace());

        self.start = self.cursor.pos();

        if self.cursor.is_at_end() {
            return Ok(self.new_token(TokenKind::Eof));
        }

        if self.cursor.check('"') {
            return self.scan_string();
        }

        if self.cursor.check_if(|c| c == '_' || c.is_ascii_alphabetic()) {
            return Ok(self.scan_ident());
        }

        if self.cursor.check_if(|c| c.is_ascii_digit()) {
            return self.scan_number();
        }

        if let Some(kind) = TokenKind::from_punct(&mut self.cursor) {
            return Ok(self.new_token(kind));
        }

        Err(LexError::UnexpectedChar(self.cursor.peek(), self.new_span()))
    }

    fn scan_string(&mut self) -> LexResult<Token> {
        assert!(self.cursor.check('"'));
        self.cursor.skip(1);

        let s = self.cursor.advance_while(|c| c != '"');

        if self.cursor.advance_if('"') {
            Ok(self.new_token(TokenKind::StrLit(s)))
        } else {
            Err(LexError::UnterminatedString(self.new_span()))
        }
    }

    fn scan_ident(&mut self) -> Token {
        assert!(self.cursor.check_if(
            |c| c.is_ascii_alphabetic() || c == '_'));

        let s = self.cursor.advance_while(
            |c| c.is_ascii_alphanumeric() || c == '_');

        let kind = match s.as_str() {
            "true" => TokenKind::BoolLit(true),
            "false" => TokenKind::BoolLit(false),
            _ => TokenKind::from_keyword(s.as_str())
                .unwrap_or(TokenKind::Ident(s))
        };

        self.new_token(kind)
    }

    fn scan_number(&mut self) -> LexResult<Token> {
        assert!(self.cursor.check_if(|c| c.is_ascii_digit()));

        let s = self.cursor.advance_while(
            |c| c.is_ascii_digit() || c == '.');

        // allow only one dot
        let dot_count = s.chars().filter(|c| *c == '.').count();

        let kind = match dot_count {
            0 => s.parse::<i64>()
                .map(TokenKind::IntLit)
                .map_err(|_| LexError::InvalidNumber(s, self.new_span()))?,

            1 => s.parse::<f64>()
                .map(TokenKind::FloatLit)
                .map_err(|_| LexError::InvalidNumber(s, self.new_span()))?,

            _ => return Err(LexError::InvalidNumber(s, self.new_span()))
        };

        Ok(self.new_token(kind))
    }
}