use ast::Node;
use ast::typed::{Func, Item};
use bytecode::{Chunk, ChunkBuilder, Instr};
use crate::compiler::Compiler;

impl Compiler<'_> {
    pub(super) fn compile_entry(&mut self) -> Chunk {
        let mut chunk = ChunkBuilder::new();

        let main = self.main_func();
        let idx = self.funcs.get(main.id());

        chunk.emit(Instr::Call(idx, 0));
        chunk.emit(Instr::Halt);

        chunk.build()
    }

    fn main_func(&self) -> &Func {
        self.program
            .iter()
            .find_map(|item| {
                match item {
                    Item::Func(node) if node.ident() == "main" => Some(node),
                    _ => None,
                }
            })
            .expect("compiler bug: found no 'main' func")
    }
}
