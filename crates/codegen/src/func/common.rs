use ast::typed::Block;
use crate::func::FuncCompiler;

impl FuncCompiler<'_> {
    pub(super) fn compile_block(&mut self, block: &Block) {
        for stmt in block {
            self.compile_stmt(stmt);
        }
    }
}
