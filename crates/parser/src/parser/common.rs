use ast::Block;
use ast::types::{Ident, TypeRef};
use lexer::Token;
use crate::parser::Parser;
use crate::{Result, Error};

impl Parser<'_> {
    pub(super) fn parse_ident(&mut self) -> Result<Ident> {
        match self.cursor.advance() {
            Some(Token::Ident(s)) => Ok(Ident::new(s)),
            Some(token) => Err(Error::UnexpectedToken(token.clone())),
            None => Err(Error::UnexpectedEof),
        }
    }

    pub(super) fn parse_type(&mut self) -> Result<TypeRef> {
        let ident = self.parse_ident()?;
        Ok(TypeRef::new(ident))
    }

    pub(super) fn parse_block(&mut self) -> Result<Block> {
        let mut stmts = Vec::new();

        match self.cursor.advance() {
            Some(Token::LBrace) => {
                while !matches!(self.cursor.peek(), Some(Token::RBrace) | None) {
                    let stmt = self.parse_stmt()?;
                    stmts.push(stmt);
                }

                expect!(self, Token::RBrace);
            },
            Some(_) => {
                let stmt = self.parse_stmt()?;
                stmts.push(stmt);
            },
            None => return Err(Error::UnexpectedEof),
        }

        Ok(stmts)
    }

    pub(super) fn parse_comma_separated<T, F>(&mut self, mut f: F) -> Result<Vec<T>>
    where
        F: FnMut(&mut Self) -> Result<T>
    {
        let mut items = Vec::new();

        while !matches!(self.cursor.peek(), Some(Token::RParen) | None) {
            items.push(f(self)?);

            match self.cursor.peek() {
                Some(Token::Comma) => self.cursor.advance(),
                None | Some(_) => break,
            };
        }

        Ok(items)
    }
}
