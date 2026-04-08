mod chunk;
mod stmt;
mod expr;
mod program;
mod common;

use crate::layout::Layout;
use crate::Result;
use ast::typed;
use bytecode::{Program, Reg};

// function local register reserved for return values
pub(crate) const RET_REG: Reg = 0;

struct Emitter<'a> {
    layout: &'a Layout,
}

impl<'a> Emitter<'a> {
    fn new(layout: &'a Layout) -> Self {
        Self { layout }
    }
    
    fn emit(self, program: &typed::Program) -> Result<Program> {
        let fns = self.emit_fns(program)?;
        let main = self.emit_entry(program)?;
        Ok(Program::new(main, fns, self.layout.global_count()))
    }
}

pub(crate) fn emit(program: &typed::Program, layout: &Layout) -> Result<Program> {
    Emitter::new(layout).emit(program)
}
