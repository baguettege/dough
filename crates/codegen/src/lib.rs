use ast::typed;
use bytecode::Program;

mod compiler;
mod func_compiler;
mod func_table;

pub fn compile(program: &typed::Program) -> Program {
    compiler::compile(program)
}
