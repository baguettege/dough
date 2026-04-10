use ast::typed;
use bytecode::Program;

mod func;
mod chunk;
mod collector;
mod entry;

pub fn compile(program: &typed::Program) -> Program {
    let table = collector::collect(program);
    let entry = entry::compile(program, &table);
    let funcs = func::compile(program, &table);
    Program::new(entry, funcs)
}
