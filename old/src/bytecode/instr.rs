use crate::bytecode::codec::{Decode, Decoder, Encode, Encoder};
use std::fmt::Formatter;
use crate::bytecode::{Reg, Idx, Off, Len, Argc};

macro_rules! define_instrs {
    ($(
        $( $doc:literal )?
        $variant:ident = $opcode:literal
        { $( $field:ident: $type:ty ),* $(,)? }
    ),+ $(,)?) => {
        /// A bytecode instruction for the Dough VM.
        pub(crate) enum Instr {
            $(
                $( #[doc=$doc] )?
                $variant {
                    $( $field: $type ),*
                }
            ),+
        }

        impl std::fmt::Display for Instr {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$variant { $( $field ),* } => {
                            write!(f, "{}", stringify!($variant))?;

                            $(
                                write!(f, " {}:{}", stringify!($field), $field)?;
                            )*
                        }
                    )+
                }

                Ok(())
            }
        }

        impl Decode for Instr {
            fn decode(decoder: &mut Decoder) -> Option<Self> {
                let opcode = decoder.decode::<$crate::bytecode::Opcode>()?;

                match opcode {
                    $(
                        $opcode => {
                            let instr = Self::$variant {
                                $( $field: decoder.decode::<$type>()? ),*
                            };

                            Some(instr)
                        },
                    )+
                    _ => None
                }
            }
        }

        impl Encode for Instr {
            fn encode(&self, encoder: &mut Encoder) {
                match self {
                    $(
                        Self::$variant { $( $field ),* } => {
                            encoder.encode(&($opcode as $crate::bytecode::Opcode));
                            $( encoder.encode($field); )*
                        },
                    )+
                }
            }
        }
    };
}

define_instrs! {
    Nop = 0x00 {},

    // int
    IAdd = 0x01 { dst: Reg, lhs: Reg, rhs: Reg },
    ISub = 0x02 { dst: Reg, lhs: Reg, rhs: Reg },
    IMul = 0x03 { dst: Reg, lhs: Reg, rhs: Reg },
    IDiv = 0x04 { dst: Reg, lhs: Reg, rhs: Reg },

    IEq = 0x05 { dst: Reg, lhs: Reg, rhs: Reg },
    INe = 0x06 { dst: Reg, lhs: Reg, rhs: Reg },
    ILe = 0x07 { dst: Reg, lhs: Reg, rhs: Reg },
    ILt = 0x08 { dst: Reg, lhs: Reg, rhs: Reg },
    IGe = 0x09 { dst: Reg, lhs: Reg, rhs: Reg },
    IGt = 0x0A { dst: Reg, lhs: Reg, rhs: Reg },

    INeg = 0x0B { dst: Reg, src: Reg },

    // float
    FAdd = 0x0C { dst: Reg, lhs: Reg, rhs: Reg },
    FSub = 0x0D { dst: Reg, lhs: Reg, rhs: Reg },
    FMul = 0x0E { dst: Reg, lhs: Reg, rhs: Reg },
    FDiv = 0x0F { dst: Reg, lhs: Reg, rhs: Reg },

    FEq = 0x10 { dst: Reg, lhs: Reg, rhs: Reg },
    FNe = 0x11 { dst: Reg, lhs: Reg, rhs: Reg },
    FLe = 0x12 { dst: Reg, lhs: Reg, rhs: Reg },
    FLt = 0x13 { dst: Reg, lhs: Reg, rhs: Reg },
    FGe = 0x14 { dst: Reg, lhs: Reg, rhs: Reg },
    FGt = 0x15 { dst: Reg, lhs: Reg, rhs: Reg },

    FNeg = 0x16 { dst: Reg, src: Reg },

    // bool
    BNot = 0x17 { dst: Reg, src: Reg },

    // str
    SEq = 0x18 { dst: Reg, lhs: Reg, rhs: Reg },
    SNe = 0x19 { dst: Reg, lhs: Reg, rhs: Reg },
    SConcat = 0x1A { dst: Reg, lhs: Reg, rhs: Reg },

    // cast
    I2F = 0x1B { dst: Reg, src: Reg },
    F2I = 0x1C { dst: Reg, src: Reg },

    // memory
    Mov = 0x1D { dst: Reg, src: Reg },
    LoadConst = 0x1E { dst: Reg, idx: Idx },
    LoadUnit = 0x1F { dst: Reg },

    // global
    LoadGlobal = 0x20 { dst: Reg, idx: Idx },
    StoreGlobal = 0x21 { idx: Idx, src: Reg },

    // jump
    Jmp = 0x22 { off: Off },
    Jf = 0x23 { cond: Reg, off: Off },

    // function
    Call = 0x24 { dst: Reg, fn_idx: Idx, arg_base: Reg, argc: Argc },
    Ret = 0x25 { src: Reg },

    // array
    ArrayNew = 0x26 { dst: Reg, len: Len },
    ArrayGet = 0x27 { dst: Reg, arr: Reg, idx: Reg },
    ArraySet = 0x28 { arr: Reg, idx: Reg, src: Reg },
    ArrayLen = 0x29 { dst: Reg, arr: Reg },
}
