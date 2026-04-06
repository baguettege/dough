use ast::typed::Block;
use ast::untyped;
use crate::Result;
use crate::typeck::TypeChecker;

impl TypeChecker<'_> {
    pub(super) fn check_block(&self, block: &untyped::Block) -> Result<Block> {
        block
            .iter()
            .map(|s| self.check_stmt(s))
            .collect()
    }
}
