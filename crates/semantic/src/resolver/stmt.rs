use ast::untyped::{Assign, ExprStmt, If, Let, Return, Stmt, While};
use crate::resolver::{ty, Resolver};
pub use crate::Result;
use crate::symbol::Symbol;

impl Resolver {
    pub(super) fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::ExprStmt(node) => self.resolve_expr_stmt(node),
            Stmt::Let(node) => self.resolve_let(node),
            Stmt::Assign(node) => self.resolve_assign(node),
            Stmt::If(node) => self.resolve_if(node),
            Stmt::While(node) => self.resolve_while(node),
            Stmt::Return(node) => self.resolve_return(node),
        }
    }

    fn resolve_expr_stmt(&mut self, node: &ExprStmt) -> Result<()> {
        self.resolve_expr(node.expr())
    }

    fn resolve_let(&mut self, node: &Let) -> Result<()> {
        // must resolve the initializer before defining the variable,
        // otherwise declarations like `let x: int = x;` are possible
        self.resolve_expr(node.init())?;

        let ty = ty::resolve(node.ty())?;
        let symbol = Symbol::Local(ty);
        self.define(node, node.ident(), symbol)?;

        Ok(())
    }

    fn resolve_assign(&mut self, node: &Assign) -> Result<()> {
        let symbol = self.scope.lookup(node.target())?.clone();
        self.bindings.insert(node, symbol);
        self.resolve_expr(node.value())?;
        Ok(())
    }

    fn resolve_if(&mut self, node: &If) -> Result<()> {
        self.resolve_expr(node.condition())?;
        self.resolve_block(node.then_body())?;

        if let Some(else_body) = node.else_body() {
            self.resolve_block(else_body)
        } else {
            Ok(())
        }
    }

    fn resolve_while(&mut self, node: &While) -> Result<()> {
        self.resolve_expr(node.condition())?;
        self.resolve_block(node.body())?;
        Ok(())
    }

    fn resolve_return(&mut self, node: &Return) -> Result<()> {
        if let Some(value) = node.value() {
            self.resolve_expr(value)
        } else {
            Ok(())
        }
    }
}
