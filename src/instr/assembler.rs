use crate::instr::Instr;

pub(crate) fn assemble(instrs: &[Instr]) -> Vec<u8> {
    let mut buf= Vec::new();
    instrs.iter().for_each(|instr| instr.encode(&mut buf));
    buf
}

#[macro_export] // Instr is pub(crate)
macro_rules! assemble {
    ( $( $instr:expr $(,)? )* ) => {
        crate::instr::assembler::assemble(&[ $( $instr, )* ])
    };
}