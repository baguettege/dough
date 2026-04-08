use ast::typed::{If, Let, Stmt, While};
use crate::allocator::Allocator;
use crate::Result;
use crate::slot::Slot;

impl Allocator {
    pub(super) fn alloc_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Let(node) => self.alloc_let(node),
            Stmt::If(node) => self.alloc_if(node),
            Stmt::While(node) => self.alloc_while(node),
            _ => Ok(()),
        }
    }

    fn alloc_let(&mut self, node: &Let) -> Result<()> {
        let reg = self.local_counter.alloc()?;
        let slot = Slot::Local(reg);
        self.slots.insert(node, slot);
        Ok(())
    }

    fn alloc_if(&mut self, node: &If) -> Result<()> {
        self.alloc_block(node.then_body())?;
        if let Some(else_body) = node.else_body() {
            self.alloc_block(else_body)?;
        }
        Ok(())
    }

    fn alloc_while(&mut self, node: &While) -> Result<()> {
        self.alloc_block(node.body())
    }
}
