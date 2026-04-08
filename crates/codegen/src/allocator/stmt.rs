use ast::typed::{Assign, ExprStmt, If, Let, Return, Stmt, While};
use crate::allocator::Allocator;
use crate::Result;
use crate::slot::Slot;

impl Allocator {
    pub(super) fn alloc_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::ExprStmt(node) => self.alloc_expr_stmt(node),
            Stmt::Let(node) => self.alloc_let(node),
            Stmt::Assign(node) => self.alloc_assign(node),
            Stmt::If(node) => self.alloc_if(node),
            Stmt::While(node) => self.alloc_while(node),
            Stmt::Return(node) => self.alloc_return(node),
        }
    }

    fn alloc_expr_stmt(&mut self, node: &ExprStmt) -> Result<()> {
        self.alloc_expr(node.expr())
    }

    fn alloc_let(&mut self, node: &Let) -> Result<()> {
        self.alloc_expr(node.init())?;

        let reg = self.local_allocator.alloc()?;
        self.slots.insert(node, Slot::Local(reg));

        Ok(())
    }

    fn alloc_assign(&mut self, node: &Assign) -> Result<()> {
        self.alloc_expr(node.value())
    }

    fn alloc_if(&mut self, node: &If) -> Result<()> {
        self.alloc_expr(node.condition())?;

        self.alloc_block(node.then_body())?;
        if let Some(else_body) = node.else_body() {
            self.alloc_block(else_body)?;
        }

        Ok(())
    }

    fn alloc_while(&mut self, node: &While) -> Result<()> {
        self.alloc_expr(node.condition())?;
        self.alloc_block(node.body())?;
        Ok(())
    }

    fn alloc_return(&mut self, node: &Return) -> Result<()> {
        if let Some(value) = node.value() {
            self.alloc_expr(value)?;
        }

        Ok(())
    }
}
