use crate::ast::*;
use crate::lexer::{Token, TokenKind};
use crate::span::Spanned;

#[derive(Debug)]
pub(crate) enum Error {
    UnexpectedToken(Token),
    ExpectedToken { expected: TokenKind, found: Token },
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

struct Cursor {
    tokens: Vec<Token>,
    pos: usize,
}

impl Cursor {
    fn new(tokens: Vec<Token>) -> Self {
        assert!(!tokens.is_empty(), "`tokens` must be non-empty");
        assert!(matches!(
            tokens.last().map(|t| t.kind()),
            Some(TokenKind::Eof)
        ));

        Self { tokens, pos: 0 }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind(), TokenKind::Eof)
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).expect("bug: cursor out of bounds")
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.pos];

        if !self.is_at_end() {
            self.pos += 1;
        }

        token
    }

    fn check(&self, kind: &TokenKind) -> bool {
        self.peek().kind() == kind
    }

    fn match_kind(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: &TokenKind) -> Result<&Token> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(Error::ExpectedToken {
                expected: kind.clone(),
                found: self.peek().clone()
            })
        }
    }
}

mod precedence {
    use crate::ast::BinaryOp;

    pub(super) const MIN: u8 = 0;

    pub(super) fn of(op: BinaryOp) -> u8 {
        match op {
            BinaryOp::Mul | BinaryOp::Div => 5,
            BinaryOp::Add | BinaryOp::Sub => 4,
            BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => 3,
            BinaryOp::Eq | BinaryOp::Ne => 2,
            BinaryOp::And => 1,
            BinaryOp::Or => 0
        }
    }
}

pub(crate) struct Parser {
    cursor: Cursor,
}

impl Parser {
    fn peek_kind(&self) -> &TokenKind {
        self.cursor.peek().kind()
    }

    fn parse_name(&mut self) -> Result<Name> {
        let token = self.cursor.advance();

        if let TokenKind::Ident(name) = token.kind() {
            Ok(Name::new(name, token.span()))
        } else {
            Err(Error::ExpectedToken {
                expected: TokenKind::Ident(String::new()),
                found: token.clone()
            })
        }
    }

    fn parse_type(&mut self) -> Result<TypeRef> {
        let name = self.parse_name()?;
        let start = name.span();

        let mut ty = TypeRef::new(
            TypeKind::Name(name),
            start
        );

        // recursively parse ...[]
        while self.cursor.match_kind(&TokenKind::LBrack) {
            let end = self.cursor.expect(&TokenKind::RBrack)?.span();

            let span = ty.span().merge(end);
            ty = TypeRef::new(
                TypeKind::Array(Box::new(ty)),
                span
            );
        }

        Ok(ty)
    }

    fn parse_literal(&mut self) -> Result<Expr> {
        let token = self.cursor.advance();

        let lit = match token.kind() {
            TokenKind::Int(v) => Literal::Int(*v),
            TokenKind::Float(v) => Literal::Float(*v),
            TokenKind::Bool(v) => Literal::Bool(*v),
            TokenKind::Str(v) => Literal::Str(v.clone()),
            _ => return Err(Error::UnexpectedToken(token.clone()))
        };

        Ok(Lit::new(lit, token.span()).into())
    }

    fn peek_binary_op(&self) -> Option<BinaryOp> {
        let op = match self.peek_kind() {
            TokenKind::Star => BinaryOp::Mul,
            TokenKind::Slash => BinaryOp::Div,

            TokenKind::Plus => BinaryOp::Add,
            TokenKind::Minus => BinaryOp::Sub,

            TokenKind::Lt => BinaryOp::Lt,
            TokenKind::Le => BinaryOp::Le,
            TokenKind::Gt => BinaryOp::Gt,
            TokenKind::Ge => BinaryOp::Ge,

            TokenKind::EqEq => BinaryOp::Eq,
            TokenKind::Ne => BinaryOp::Ne,

            TokenKind::And => BinaryOp::And,

            TokenKind::Or => BinaryOp::Or,

            _ => return None
        };

        Some(op)
    }

    fn peek_unary_op(&self) -> Option<UnaryOp> {
        let op = match self.peek_kind() {
            TokenKind::Not => UnaryOp::Not,
            TokenKind::Minus => UnaryOp::Neg,
            _ => return None
        };

        Some(op)
    }
}

impl Parser {
    pub(crate) fn parse(tokens: Vec<Token>) -> Result<Program> {
        Self::new(tokens).parse_program()
    }

    fn new(tokens: Vec<Token>) -> Self {
        Self { cursor: Cursor::new(tokens) }
    }

    fn parse_program(mut self) -> Result<Program> {
        let mut decls = Vec::new();

        while !self.cursor.is_at_end() {
            decls.push(self.parse_decl()?);
        }

        Ok(Program::new(decls))
    }

    fn parse_decl(&mut self) -> Result<Decl> {
        let token = self.cursor.peek();

        match token.kind() {
            TokenKind::Func => self.parse_func().map(Decl::from),
            TokenKind::Var => self.parse_var().map(Decl::from),

            _ => Err(Error::UnexpectedToken(token.clone()))
        }
    }

    fn parse_func(&mut self) -> Result<Func> {
        // func name(params...): return_type { body }

        let start = self.cursor.expect(&TokenKind::Func)?.span();

        let name = self.parse_name()?;
        self.cursor.expect(&TokenKind::LParen)?;

        let mut params = Vec::new();

        if !self.cursor.check(&TokenKind::RParen) {
            loop {
                let name = self.parse_name()?;
                self.cursor.expect(&TokenKind::Colon)?;

                let type_ref = self.parse_type()?;

                let span = name.span().merge(type_ref.span());
                params.push(Param::new(name, type_ref, span));

                if !self.cursor.match_kind(&TokenKind::Comma) {
                    break;
                }
            }
        }

        self.cursor.expect(&TokenKind::RParen)?;
        self.cursor.expect(&TokenKind::Colon)?;

        let return_type = self.parse_type()?;

        let body = self.parse_block()?;
        let span = start.merge(body.span());

        Ok(Func::new(name, params, return_type, body, span))
    }

    fn parse_var(&mut self) -> Result<Var> {
        // var name: type = init;

        let start = self.cursor.expect(&TokenKind::Var)?.span();

        let name = self.parse_name()?;
        self.cursor.expect(&TokenKind::Colon)?;

        let type_ref = self.parse_type()?;
        self.cursor.expect(&TokenKind::Eq)?;

        let init = self.parse_expr()?;
        self.cursor.expect(&TokenKind::Semicolon)?;

        let span = start.merge(init.span());

        Ok(Var::new(name, type_ref, init, span))
    }

    fn parse_stmt(&mut self) -> Result<Stmt> {
        let token = self.cursor.peek();

        match token.kind() {
            TokenKind::If => self.parse_if().map(Stmt::from),
            TokenKind::While => self.parse_while().map(Stmt::from),
            TokenKind::Return => self.parse_return().map(Stmt::from),

            TokenKind::Var => self.parse_var()
                .map(Decl::from)
                .map(|decl| {
                    let span = decl.span();
                    DeclStmt::new(decl, span)
                })
                .map(Stmt::from),

            _ => self.parse_assign_or_expr()
        }
    }

    fn parse_block(&mut self) -> Result<Block> {
        // allows single stmt block without braces
        // like `if cond stmt`

        let start = self.cursor.peek().span();
        let mut stmts = Vec::new();

        if self.cursor.match_kind(&TokenKind::LBrace) {
            while !self.cursor.check(&TokenKind::RBrace) {
                stmts.push(self.parse_stmt()?);
            }

            let end = self.cursor.expect(&TokenKind::RBrace)?.span();
            Ok(Block::new(stmts, start.merge(end)))
        } else {
            let stmt = self.parse_stmt()?;
            let span = start.merge(stmt.span());

            stmts.push(stmt);
            Ok(Block::new(stmts, span))
        }
    }

    fn parse_if(&mut self) -> Result<If> {
        let start = self.cursor.expect(&TokenKind::If)?.span();

        let condition = self.parse_expr()?;
        let then_body = self.parse_block()?;

        let (else_body, end) =
            if self.cursor.match_kind(&TokenKind::Else) {

            let block = self.parse_block()?;
            let end = block.span();
            (Some(block), end)
        } else {
            (None, then_body.span())
        };

        Ok(If::new(condition, then_body, else_body, start.merge(end)))
    }

    fn parse_while(&mut self) -> Result<While> {
        let start = self.cursor.expect(&TokenKind::While)?.span();

        let condition = self.parse_expr()?;
        let body = self.parse_block()?;

        let span = start.merge(body.span());

        Ok(While::new(condition, body, span))
    }

    fn parse_return(&mut self) -> Result<Return> {
        let start = self.cursor.expect(&TokenKind::Return)?.span();

        let value = if self.cursor.check(&TokenKind::Semicolon) {
            None
        } else {
            Some(self.parse_expr()?)
        };

        let end = self.cursor.expect(&TokenKind::Semicolon)?.span();

        Ok(Return::new(value, start.merge(end)))
    }

    fn parse_assign_or_expr(&mut self) -> Result<Stmt> {
        let expr = self.parse_expr()?;
        let span = expr.span();

        if self.cursor.match_kind(&TokenKind::Eq) {
            let value = self.parse_expr()?;
            let span = span.merge(value.span());

            self.cursor.expect(&TokenKind::Semicolon)?;
            Ok(Assign::new(expr, value, span).into())
        } else {
            self.cursor.expect(&TokenKind::Semicolon)?;
            Ok(ExprStmt::new(expr, span).into())
        }
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        let operand = self.parse_operand()?;
        self.parse_expr_inner(operand, precedence::MIN)
    }

    fn parse_expr_inner(&mut self, mut lhs: Expr, min_precedence: u8) -> Result<Expr> {
        // continue consuming binary ops as long as they bind tighter
        // than the current `min_precedence`
        while let Some(op) = self.peek_binary_op() {
            let precedence = precedence::of(op);

            // stop if this op would bind too weak for this expr
            if precedence <= min_precedence {
                break;
            } else {
                self.cursor.advance();
            }

            // parse the rhs operand and recursively parse to allow
            // higher precedence ops to bind
            // note: this assumes all ops are left associative
            let rhs = self.parse_operand()?;
            let rhs = self.parse_expr_inner(rhs, precedence)?;

            // now combine the two into a binary expr
            let span = lhs.span().merge(rhs.span());
            let binary = Binary::new(
                Box::new(lhs),
                op,
                Box::new(rhs),
                span
            );

            lhs = Expr::Binary(binary);
        }

        Ok(lhs)
    }

    fn parse_operand(&mut self) -> Result<Expr> {
        let mut expr = if let Some(op) = self.peek_unary_op() {
            let start = self.cursor.advance().span();

            let expr = self.parse_operand()?;
            let span = start.merge(expr.span());

            Unary::new(op, Box::new(expr), span).into()
        } else {
            self.parse_primary()?
        };

        expr = self.parse_postfix(expr)?;
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        match self.peek_kind() {
            TokenKind::LParen => self.parse_group(),

            TokenKind::Ident(_) => self.parse_ident().map(Expr::from),

            // note: this creates 2 sources of truth for what is a literal,
            // here and in `parse_literal`, but it's a tradeoff that
            // can't be avoided easily
            TokenKind::Int(_) |
            TokenKind::Float(_) |
            TokenKind::Str(_) |
            TokenKind::Bool(_) => self.parse_literal(),

            _ => Err(Error::UnexpectedToken(
                self.cursor.peek().clone()))
        }
    }

    fn parse_group(&mut self) -> Result<Expr> {
        self.cursor.expect(&TokenKind::LParen)?;
        let expr = self.parse_expr()?;
        self.cursor.expect(&TokenKind::RParen)?;
        Ok(expr)
    }

    fn parse_ident(&mut self) -> Result<Ident> {
        let name = self.parse_name()?;
        let span = name.span();
        Ok(Ident::new(name, span))
    }

    fn parse_postfix(&mut self, mut expr: Expr) -> Result<Expr> {
        loop {
            expr = match self.peek_kind() {
                TokenKind::LParen => self.parse_call(expr)?.into(),
                TokenKind::LBrack => self.parse_index(expr)?.into(),
                TokenKind::Dot => self.parse_member(expr)?.into(),
                _ => break
            }
        }

        Ok(expr)
    }

    fn parse_call(&mut self, expr: Expr) -> Result<Call> {
        self.cursor.expect(&TokenKind::LParen)?;
        let mut args = Vec::new();

        if !self.cursor.check(&TokenKind::RParen) {
            loop {
                let arg = self.parse_expr()?;
                args.push(arg);

                if !self.cursor.match_kind(&TokenKind::Comma) {
                    break;
                }
            }
        }

        let end = self.cursor.expect(&TokenKind::RParen)?.span();
        let span = expr.span().merge(end);

        Ok(Call::new(
            Box::new(expr),
            args,
            span
        ))
    }

    fn parse_index(&mut self, expr: Expr) -> Result<Index> {
        self.cursor.expect(&TokenKind::LBrack)?;

        let index = self.parse_expr()?;

        let end = self.cursor.expect(&TokenKind::RBrack)?.span();
        let span = expr.span().merge(end);

        Ok(Index::new(
            Box::new(expr),
            Box::new(index),
            span
        ))
    }

    fn parse_member(&mut self, expr: Expr) -> Result<Member> {
        self.cursor.expect(&TokenKind::Dot)?;

        let member = self.parse_name()?;
        let span = expr.span().merge(member.span());

        Ok(Member::new(
            Box::new(expr),
            member,
            span
        ))
    }
}
