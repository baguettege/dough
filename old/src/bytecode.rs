pub(crate) use codec::{Decode, Decoder, Encode, Encoder};
pub(crate) use instr::Instr;
pub(crate) use types::{Opcode, Reg, Idx, Off, Len, Argc};

mod codec;
mod instr;
mod types;
mod chunk;

macro_rules! bytecode {
    ($(
        $instr:ident $( $operand:ident: $value:expr )* ;
    )*) => {
        {
            let mut encoder = $crate::bytecode::Encoder::new();

            $(
                let instr = $crate::bytecode::Instr::$instr {
                    $( $operand: $value ),*
                };

                encoder.encode(&instr);
            )*

            encoder.finish()
        }
    };
}

pub(crate) use bytecode;
