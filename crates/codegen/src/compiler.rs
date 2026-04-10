mod collector;
mod func;
mod entry;

use ast::typed;
use bytecode::{Program};
use crate::func_table::FuncTable;

struct Compiler<'a> {
    program: &'a typed::Program,
    funcs: &'a FuncTable,
}

impl<'a> Compiler<'a> {
    fn new(program: &'a typed::Program, funcs: &'a FuncTable) -> Self {
        Self { program, funcs }
    }

    fn compile(mut self) -> Program {
        let entry = self.compile_entry();
        let funcs = self.compile_funcs();
        Program::new(entry, funcs)
    }
}

pub(crate) fn compile(program: &typed::Program) -> Program {
    let funcs = collector::collect(program);
    Compiler::new(program, &funcs).compile()
}
