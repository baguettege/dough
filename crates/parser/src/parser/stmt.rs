use ast::Stmt;
use lexer::Token;
use crate::parser::Parser;
use crate::{Result, Error};

impl Parser<'_> {
    pub(super) fn parse_stmt(&mut self) -> Result<Stmt> {
        match self.cursor.peek() {
            Some(Token::Let) => self.parse_let(),
            Some(Token::If) => self.parse_if(),
            Some(Token::While) => self.parse_while(),
            Some(Token::Return) => self.parse_return(),
            Some(_) => self.parse_expr_stmt(),
            None => Err(Error::UnexpectedEof),
        }
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt> {
        let expr = self.parse_expr()?;
        expect!(self, Token::Semicolon);
        Ok(Stmt::Expr(expr))
    }

    fn parse_let(&mut self) -> Result<Stmt> {
        expect!(self, Token::Let);
        let ident = self.parse_ident()?;
        expect!(self, Token::Colon);
        let ty = self.parse_type()?;
        expect!(self, Token::Assign);
        let init = self.parse_expr()?;
        expect!(self, Token::Semicolon);

        Ok(Stmt::Let { ident, ty, init })
    }

    fn parse_if(&mut self) -> Result<Stmt> {
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

        Ok(Stmt::If { condition, then_body, else_body })
    }

    fn parse_while(&mut self) -> Result<Stmt> {
        expect!(self, Token::While);
        let condition = self.parse_expr()?;
        let body = self.parse_block()?;

        Ok(Stmt::While { condition, body })
    }

    fn parse_return(&mut self) -> Result<Stmt> {
        expect!(self, Token::Return);
        let value =
            (!matches!(self.cursor.peek(), Some(Token::Semicolon)))
                .then(|| self.parse_expr())
                .transpose()?;
        expect!(self, Token::Semicolon);

        Ok(Stmt::Return { value })
    }
}
