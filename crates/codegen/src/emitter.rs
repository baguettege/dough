mod chunk;

use ast::typed;
use bytecode::{Encoder, Program};
use crate::slot;

struct Emitter<'a> {
    slots: &'a slot::Table,
    encoder: Encoder,
}

impl<'a> Emitter<'a> {
    fn new(slots: &'a slot::Table) -> Self {
        let encoder = Encoder::new();
        Self { slots, encoder }
    }
    
    fn emit(mut self, program: &typed::Program) -> Program {
        todo!()
    }
}

pub(crate) fn emit(program: &typed::Program, slots: &slot::Table) -> Program {
    Emitter::new(slots).emit(program)
}
