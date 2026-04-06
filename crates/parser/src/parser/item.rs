use ast::untyped::{Item, Param, Fn, Static};
use lexer::Token;
use crate::parser::Parser;
use crate::{Result, Error};

impl Parser<'_> {
    pub(super) fn parse_item(&mut self) -> Result<Item> {
        match self.cursor.peek() {
            Some(Token::Fn) => self.parse_fn().map(Into::into),
            Some(Token::Static) => self.parse_static().map(Into::into),
            Some(token) => Err(Error::UnexpectedToken(token.clone())),
            None => Err(Error::UnexpectedEof),
        }
    }

    fn parse_fn(&mut self) -> Result<Fn> {
        expect!(self, Token::Fn);
        let ident = self.parse_ident()?;
        expect!(self, Token::LParen);

        let params = self.parse_comma_separated(|this| {
            let ident = this.parse_ident()?;
            expect!(this, Token::Colon);
            let ty = this.parse_type()?;
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

        Ok(Fn::new(self.next_id(), ident, params, return_ty, body))
    }

    fn parse_static(&mut self) -> Result<Static> {
        expect!(self, Token::Static);
        let ident = self.parse_ident()?;
        expect!(self, Token::Colon);
        let ty = self.parse_type()?;
        expect!(self, Token::Assign);
        let init = self.parse_expr()?;
        expect!(self, Token::Semicolon);
        
        Ok(Static::new(self.next_id(), ident, ty, init))
    }
}
