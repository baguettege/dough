use ast::typed::{Func, Item, Program};
use bytecode::Chunk;
use alloc::LocalAllocator;
use crate::collector::FuncTable;
use crate::chunk::Builder;

mod expr;
mod stmt;
mod common;
mod alloc;

struct FuncCompiler<'a> {
    chunk: Builder,
    locals: LocalAllocator,
    funcs: &'a FuncTable,
}

impl<'a> FuncCompiler<'a> {
    fn new(funcs: &'a FuncTable) -> Self {
        let chunk = Builder::new();
        let locals = LocalAllocator::new();
        Self { chunk, locals, funcs }
    }

    fn compile(mut self, node: &Func) -> Chunk {
        for param in node.params() {
            let _ = self.locals.alloc(param);
        }

        self.compile_block(node.body());
        let local_count = self.locals.count();
        self.chunk.build(local_count)
    }
}

pub(crate) fn compile(program: &Program, funcs: &FuncTable) -> Vec<Chunk> {
    program
        .iter()
        .filter_map(|item| match item {
            Item::Func(node) =>
                Some(FuncCompiler::new(funcs).compile(node)),
        })
        .collect()
}


