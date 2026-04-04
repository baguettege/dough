use crate::cursor::Cursor;
use crate::{Error, Result, Token};
use crate::scan;

pub(crate) struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self { cursor: Cursor::new(input) }
    }

    pub(crate) fn lex(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.cursor.is_at_end() {
            self.cursor.advance_while(|c| c.is_whitespace());
            if self.cursor.is_at_end() { break; }

            let token = self.scan_token()?;
            tokens.push(token);
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token> {
        if let Some(token) = scan::from_op(&mut self.cursor)
            .or_else(|| scan::from_punct(&mut self.cursor)) {
            return Ok(token);
        }

        match self.cursor.peek().ok_or(Error::UnexpectedEof)? {
            c if c.is_alphabetic() || c == '_' => self.scan_ident(),
            c if c.is_ascii_digit() => self.scan_number(),
            '"' => self.scan_str(),
            c => Err(Error::UnexpectedChar(c)),
        }
    }

    fn scan_ident(&mut self) -> Result<Token> {
        let s = self.cursor.advance_while(
            |c| c.is_alphanumeric() || c == '_');
        Ok(scan::from_keyword(s).unwrap_or_else(
            || Token::Ident(s.to_string())))
    }

    fn scan_number(&mut self) -> Result<Token> {
        let s = self.cursor.advance_while(
            |c| c.is_ascii_digit() || c == '.');

        let dots = s
            .chars()
            .filter(|&c| c == '.')
            .count();

        match dots {
            0 => s
                .parse::<i64>()
                .map(|i| Token::Int(i))
                .map_err(|_| Error::InvalidNumber(s.to_string())),
            1 => s
                .parse::<f64>()
                .map(|f| Token::Float(f))
                .map_err(|_| Error::InvalidNumber(s.to_string())),
            _ => Err(Error::InvalidNumber(s.to_string())),
        }
    }

    fn scan_str(&mut self) -> Result<Token> {
        self.cursor.expect('"')?;
        let s = self.cursor.advance_while(|
            c| c != '"').to_string();
        self.cursor.expect('"')?;
        Ok(Token::Str(s))
    }
}
