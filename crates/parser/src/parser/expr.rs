use ast::untyped::Expr;
use ast::types::{BinOp, Literal, UnOp};
use lexer::Token;
use crate::parser::Parser;
use crate::{Result, Error};

mod precedence {
    use ast::types::BinOp;

    pub(super) const MIN: u32 = 0;

    pub(super) fn of(op: BinOp) -> u32 {
        match op {
            BinOp::Mul | BinOp::Div => 6,
            BinOp::Add | BinOp::Sub => 5,
            BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => 4,
            BinOp::Eq | BinOp::Ne => 3,
            BinOp::And => 2,
            BinOp::Or => 1,
        }
    }
}

impl Parser<'_> {
    pub(super) fn parse_expr(&mut self) -> Result<Expr> {
        let lhs = self.parse_operand()?;
        self.parse_expr_inner(lhs, precedence::MIN)
    }

    fn parse_expr_inner(&mut self, mut lhs: Expr, min_precedence: u32) -> Result<Expr> {
        while let Some(op) = self.peek_binop() {
            let precedence = precedence::of(op);
            if precedence < min_precedence { break; }
            self.cursor.advance();

            let rhs = self.parse_operand()?;
            let rhs = self.parse_expr_inner(rhs, precedence + 1)?;

            lhs = Expr::Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_operand(&mut self) -> Result<Expr> {
        match self.peek_unop() {
            Some(op) => {
                self.cursor.advance();
                let expr = self.parse_operand()?;
                Ok(Expr::Unary {
                    op,
                    expr: Box::new(expr),
                })
            },
            None => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        match self.cursor.peek() {
            Some(Token::Int(_) | Token::Float(_) | Token::Bool(_) |
                 Token::Str(_)) => self.parse_literal(),
            Some(Token::LParen) => self.parse_grouped(),
            Some(Token::Ident(_)) => {
                if matches!(self.cursor.peek_ahead(1), Some(Token::LParen)) {
                    self.parse_call()
                } else {
                    self.parse_ident_expr()
                }
            },
            Some(token) => Err(Error::UnexpectedToken(token.clone())),
            None => Err(Error::UnexpectedEof),
        }
    }

    fn parse_literal(&mut self) -> Result<Expr> {
        let literal = match self.cursor.advance() {
            Some(Token::Int(i)) => Literal::Int(*i),
            Some(Token::Float(f)) => Literal::Float(*f),
            Some(Token::Bool(b)) => Literal::Bool(*b),
            Some(Token::Str(s)) => Literal::Str(s.clone()),
            Some(token) => return Err(Error::UnexpectedToken(token.clone())),
            None => return Err(Error::UnexpectedEof),
        };

        Ok(Expr::Literal(literal))
    }

    fn parse_grouped(&mut self) -> Result<Expr> {
        expect!(self, Token::LParen);
        let expr = self.parse_expr()?;
        expect!(self, Token::RParen);
        Ok(expr)
    }

    fn parse_ident_expr(&mut self) -> Result<Expr> {
        let ident = self.parse_ident()?;
        Ok(Expr::Ident(ident))
    }

    fn parse_call(&mut self) -> Result<Expr> {
        let callee = self.parse_ident()?;

        expect!(self, Token::LParen);
        let args = self.parse_comma_separated(
            |this| this.parse_expr())?;
        expect!(self, Token::RParen);

        Ok(Expr::Call { callee, args })
    }
}

impl Parser<'_> {
    fn peek_binop(&self) -> Option<BinOp> {
        match self.cursor.peek() {
            Some(Token::Star) => Some(BinOp::Mul),
            Some(Token::Slash) => Some(BinOp::Div),
            Some(Token::Plus) => Some(BinOp::Add),
            Some(Token::Minus) => Some(BinOp::Sub),
            Some(Token::Lt) => Some(BinOp::Lt),
            Some(Token::Le) => Some(BinOp::Le),
            Some(Token::Gt) => Some(BinOp::Gt),
            Some(Token::Ge) => Some(BinOp::Ge),
            Some(Token::Eq) => Some(BinOp::Eq),
            Some(Token::Ne) => Some(BinOp::Ne),
            Some(Token::And) => Some(BinOp::And),
            Some(Token::Or) => Some(BinOp::Or),
            _ => None,
        }
    }

    fn peek_unop(&self) -> Option<UnOp> {
        match self.cursor.peek() {
            Some(Token::Not) => Some(UnOp::Not),
            Some(Token::Minus) => Some(UnOp::Neg),
            _ => None,
        }
    }
}
