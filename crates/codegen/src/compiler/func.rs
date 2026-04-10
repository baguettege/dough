use crate::compiler::Compiler;
use crate::func_compiler;
use ast::typed::Item;
use bytecode::Chunk;

impl Compiler<'_> {
    pub(super) fn compile_funcs(&self) -> Vec<Chunk> {
        self.program
            .iter()
            .filter_map(|item| {
                match item {
                    Item::Func(node) =>
                        Some(func_compiler::compile(self.funcs, node))
                }
            })
            .collect()
    }
}
