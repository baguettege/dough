use crate::chunk::Builder;
use crate::collector::FuncTable;
use ast::typed::{Func, Item, Program};
use ast::Node;
use bytecode::{Chunk, Instr};

pub(crate) fn compile(program: &Program, funcs: &FuncTable) -> Chunk {
    let main = main_func(program);
    let idx = funcs.get(main.id());

    let mut builder = Builder::new();
    builder.emit(Instr::Call(idx, 0));
    builder.emit(Instr::Halt);
    builder.build()
}

fn main_func(program: &Program) -> &Func {
    program
        .iter()
        .find_map(|item| match item {
            Item::Func(node) if node.ident() == "main" => Some(node),
            _ => None,
        })
        .expect("compiler bug: no 'main' func")
}
