use std::rc::Rc;
use crate::instr::{Decoder, Instr};

pub(crate) fn disassemble(code: Vec<u8>) -> Vec<Instr> {
    let mut output = Vec::new();
    let mut decoder = Decoder::new(Rc::new(code));

    while decoder.has_next() {
        let instr = decoder.next_instr();
        output.push(instr);
    }

    output
}