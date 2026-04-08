use ast::typed::{Binary, Call, Expr, Ident, LiteralExpr, Unary};
use crate::allocator::Allocator;
use crate::Result;
use crate::slot::Slot;

impl Allocator {
    pub(super) fn alloc_expr(&mut self, expr: &Expr) -> Result<()> {
        match expr {
            Expr::LiteralExpr(node) => self.alloc_literal_expr(node),
            Expr::Ident(node) => self.alloc_ident(node),
            Expr::Binary(node) => self.alloc_binary(node),
            Expr::Unary(node) => self.alloc_unary(node),
            Expr::Call(node) => self.alloc_call(node),
        }
    }

    fn alloc_literal_expr(&mut self, node: &LiteralExpr) -> Result<()> {
        let dst = self.local_allocator.alloc()?;
        self.slots.insert(node, Slot::Local(dst));
        Ok(())
    }

    fn alloc_ident(&mut self, node: &Ident) -> Result<()> {
        let dst = self.local_allocator.alloc()?;
        self.slots.insert(node, Slot::Local(dst));
        Ok(())
    }

    fn alloc_binary(&mut self, node: &Binary) -> Result<()> {
        self.alloc_expr(node.lhs())?;
        self.alloc_expr(node.rhs())?;

        let dst = self.local_allocator.alloc()?;
        self.slots.insert(node, Slot::Local(dst));

        Ok(())
    }

    fn alloc_unary(&mut self, node: &Unary) -> Result<()> {
        self.alloc_expr(node.expr())?;

        let dst = self.local_allocator.alloc()?;
        self.slots.insert(node, Slot::Local(dst));

        Ok(())
    }

    fn alloc_call(&mut self, node: &Call) -> Result<()> {
        for arg in node.args() {
            self.alloc_expr(arg)?;
        }

        // `alloc_range` reserves slots for the emitter
        let dst = self.local_allocator.alloc_range(node.args().len() + 1)?;
        self.slots.insert(node, Slot::Local(dst));

        Ok(())
    }
}
