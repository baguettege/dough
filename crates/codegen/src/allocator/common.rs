use ast::typed::Block;
use crate::allocator::Allocator;
use crate::Result;

impl Allocator {
    pub(super) fn alloc_block(&mut self, block: &Block) -> Result<()> {
        block.iter().try_for_each(|stmt| self.alloc_stmt(stmt))
    }
}
