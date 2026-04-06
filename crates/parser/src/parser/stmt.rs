use ast::untyped::{Assign, ExprStmt, If, Let, Return, Stmt, While};
use lexer::Token;
use crate::parser::Parser;
use crate::{Result, Error};

impl Parser<'_> {
    pub(super) fn parse_stmt(&mut self) -> Result<Stmt> {
        match self.cursor.peek() {
            Some(Token::Let) => self.parse_let().map(Into::into),
            Some(Token::If) => self.parse_if().map(Into::into),
            Some(Token::While) => self.parse_while().map(Into::into),
            Some(Token::Return) => self.parse_return().map(Into::into),
            Some(_) => self.parse_assign_or_expr_stmt(),
            None => Err(Error::UnexpectedEof),
        }
    }

    fn parse_assign_or_expr_stmt(&mut self) -> Result<Stmt> {
        if matches!(self.cursor.peek_ahead(1), Some(Token::Assign)) {
            self.parse_assign().map(Into::into)
        } else {
            self.parse_expr_stmt().map(Into::into)
        }
    }

    fn parse_assign(&mut self) -> Result<Assign> {
        let target = self.parse_ident()?;
        expect!(self, Token::Assign);
        let value = self.parse_expr()?;
        expect!(self, Token::Semicolon);
        
        Ok(Assign::new(self.next_id(), target, value))
    }

    fn parse_expr_stmt(&mut self) -> Result<ExprStmt> {
        let expr = self.parse_expr()?;
        expect!(self, Token::Semicolon);

        Ok(ExprStmt::new(self.next_id(), expr))
    }

    fn parse_let(&mut self) -> Result<Let> {
        expect!(self, Token::Let);
        let ident = self.parse_ident()?;
        expect!(self, Token::Colon);
        let ty = self.parse_type()?;
        expect!(self, Token::Assign);
        let init = self.parse_expr()?;
        expect!(self, Token::Semicolon);

        Ok(Let::new(self.next_id(), ident, ty, init))
    }

    fn parse_if(&mut self) -> Result<If> {
        expect!(self, Token::If);
        let condition = self.parse_expr()?;
        let then_body = self.parse_block()?;
        let else_body =
            matches!(self.cursor.peek(), Some(Token::Else))
                .then(|| {
                self.cursor.advance();
                self.parse_block()
                })
                .transpose()?;

        Ok(If::new(self.next_id(), condition, then_body, else_body))
    }

    fn parse_while(&mut self) -> Result<While> {
        expect!(self, Token::While);
        let condition = self.parse_expr()?;
        let body = self.parse_block()?;

        Ok(While::new(self.next_id(), condition, body))
    }

    fn parse_return(&mut self) -> Result<Return> {
        expect!(self, Token::Return);
        let value =
            (!matches!(self.cursor.peek(), Some(Token::Semicolon)))
                .then(|| self.parse_expr())
                .transpose()?;
        expect!(self, Token::Semicolon);

        Ok(Return::new(self.next_id(), value))
    }
}
