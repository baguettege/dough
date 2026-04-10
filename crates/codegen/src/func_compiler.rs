use ast::typed::Func;
use bytecode::{Chunk, ChunkBuilder};
use alloc::LocalAllocator;
use crate::func_table::FuncTable;

mod expr;
mod stmt;
mod common;
mod alloc;

struct FuncCompiler<'a> {
    chunk: ChunkBuilder,
    locals: LocalAllocator,
    funcs: &'a FuncTable,
}

impl<'a> FuncCompiler<'a> {
    fn new(funcs: &'a FuncTable) -> Self {
        let chunk = ChunkBuilder::new();
        let locals = LocalAllocator::new();
        Self { chunk, locals, funcs }
    }

    fn compile(mut self, node: &Func) -> Chunk {
        for param in node.params() {
            let _ = self.locals.alloc(param);
        }

        self.compile_block(node.body());
        self.chunk.build()
    }
}

pub(crate) fn compile(funcs: &FuncTable, node: &Func) -> Chunk {
    FuncCompiler::new(funcs).compile(node)
}
