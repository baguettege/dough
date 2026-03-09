use crate::instr::{Decoder, Instr};

pub(crate) fn disassemble(code: &[u8]) -> Vec<Instr> {
    let mut output = Vec::new();
    let mut decoder = Decoder::new(code);

    while decoder.has_next() {
        let instr = decoder.next_instr();
        output.push(instr);
    }

    output
}