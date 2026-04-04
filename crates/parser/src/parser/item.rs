use ast::Item;
use ast::types::Param;
use lexer::Token;
use crate::parser::Parser;
use crate::{Result, Error};

impl Parser<'_> {
    pub(super) fn parse_item(&mut self) -> Result<Item> {
        match self.cursor.peek() {
            Some(Token::Fn) => self.parse_fn(),
            Some(Token::Static) => self.parse_static(),
            Some(token) => Err(Error::UnexpectedToken(token.clone())),
            None => Err(Error::UnexpectedEof),
        }
    }

    fn parse_fn(&mut self) -> Result<Item> {
        expect!(self, Token::Fn);
        let ident = self.parse_ident()?;
        expect!(self, Token::LParen);

        let params = self.parse_comma_separated(|parser| {
            let ident = parser.parse_ident()?;
            expect!(parser, Token::Colon);
            let ty = parser.parse_type()?;
            Ok(Param::new(ident, ty))
        })?;

        expect!(self, Token::RParen);

        let return_ty =
            matches!(self.cursor.peek(), Some(Token::Colon))
                .then(|| {
                self.cursor.advance();
                self.parse_type()
                })
                .transpose()?;

        let body = self.parse_block()?;

        Ok(Item::Fn { ident, params, return_ty, body })
    }

    fn parse_static(&mut self) -> Result<Item> {
        expect!(self, Token::Static);
        let ident = self.parse_ident()?;
        expect!(self, Token::Colon);
        let ty = self.parse_type()?;
        expect!(self, Token::Assign);
        let init = self.parse_expr()?;
        expect!(self, Token::Semicolon);

        Ok(Item::Static { ident, ty, init })
    }
}
