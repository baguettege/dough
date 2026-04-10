use ast::untyped::{Item, Param, Func};
use lexer::Token;
use crate::parser::Parser;
use crate::{Result, Error};

impl Parser<'_> {
    pub(super) fn parse_item(&mut self) -> Result<Item> {
        match self.cursor.peek() {
            Some(Token::Func) => self.parse_func().map(Into::into),
            Some(token) => Err(Error::UnexpectedToken(token.clone())),
            None => Err(Error::UnexpectedEof),
        }
    }

    fn parse_func(&mut self) -> Result<Func> {
        expect!(self, Token::Func);
        let ident = self.parse_ident()?;
        expect!(self, Token::LParen);

        let params = self.parse_comma_separated(|this| {
            let ident = this.parse_ident()?;
            expect!(this, Token::Colon);
            let ty = this.parse_type()?;
            Ok(Param::new(this.next_id(), ident, ty))
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

        Ok(Func::new(self.next_id(), ident, params, return_ty, body))
    }
}
