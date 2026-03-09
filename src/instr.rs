pub(crate) mod disassembler;
pub(crate) mod assembler;
mod encoder;
mod decoder;

use crate::instr::decoder::{Decoder, Decode};
use crate::instr::encoder::Encode;
use std::fmt;
use std::fmt::{Display, Formatter};

macro_rules! define_instrs {
    (
        $(
            $mnemonic:ident ( $opcode:literal ) {
                $( $operand:ident: $ty:ty ),* $(,)?
            }
        )+
    ) => {
        #[derive(Debug)]
        pub(crate) enum Instr {
            $(
                $mnemonic {
                    $( $operand: $ty, )*
                },
            )*
        }

        impl Instr {
            fn decode(decoder: &mut Decoder) -> Self {
                let op = decoder.next_u8();

                match op {
                    $(
                        $opcode => Instr::$mnemonic {
                            $( $operand: <$ty as Decode>::decode(decoder), )*
                        },
                    )*
                    _ => unreachable!("unknown opcode {}", op)
                }
            }

            fn encode(&self, buf: &mut Vec<u8>) {
                match self {
                    $(
                        Instr::$mnemonic { $( $operand, )* } => {
                            buf.push($opcode);
                            $( $operand.encode(buf); )*
                        }
                    )+
                }
            }
        }

        impl Display for Instr {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                match self {
                    $(
                        Instr::$mnemonic { $( $operand, )* } => {
                            write!(f, stringify!($mnemonic))?;
                            $(
                                write!(f, " {}:{}", stringify!($operand), $operand)?;
                            )*
                            Ok(())
                        }
                    )*
                }
            }
        }
    };
}

define_instrs! {
    Nop(0x00) {}

   // int
    IAdd(0x01) { dst: u8, a: u8, b: u8 }
    ISub(0x02) { dst: u8, a: u8, b: u8 }
    IMul(0x03) { dst: u8, a: u8, b: u8 }
    IDiv(0x04) { dst: u8, a: u8, b: u8 }

    IEq(0x05) { dst: u8, a: u8, b: u8 }
    INe(0x06) { dst: u8, a: u8, b: u8 }
    ILt(0x07) { dst: u8, a: u8, b: u8 }
    ILe(0x08) { dst: u8, a: u8, b: u8 }
    IGt(0x09) { dst: u8, a: u8, b: u8 }
    IGe(0x0A) { dst: u8, a: u8, b: u8 }

    INeg(0x11) { dst: u8, src: u8}

    // float
    FAdd(0x12) { dst: u8, a: u8, b: u8 }
    FSub(0x13) { dst: u8, a: u8, b: u8 }
    FMul(0x14) { dst: u8, a: u8, b: u8 }
    FDiv(0x15) { dst: u8, a: u8, b: u8 }

    FEq(0x16) { dst: u8, a: u8, b: u8 }
    FNe(0x17) { dst: u8, a: u8, b: u8 }
    FLt(0x18) { dst: u8, a: u8, b: u8 }
    FLe(0x19) { dst: u8, a: u8, b: u8 }
    FGt(0x1A) { dst: u8, a: u8, b: u8 }
    FGe(0x1B) { dst: u8, a: u8, b: u8 }

    FNeg(0x1C) { dst: u8, src: u8 }

    // bool
    Not(0x1D) { dst: u8, src: u8 }

    // str
    SEq(0x1E) { dst: u8, a: u8, b: u8 }
    SNe(0x1F) { dst: u8, a: u8, b: u8 }
    Concat(0x20) { dst: u8, a: u8, b: u8 }

    // cast
    I2F(0x21) { dst: u8, src: u8 }
    F2I(0x22) { dst: u8, src: u8 }

    // memory
    Mov(0x23) { dst: u8, src: u8 }
    LoadConst(0x24) { dst: u8, idx: u16 }
    LoadUnit(0x25) { dst: u8 }

    // global
    GetGlobal(0x26) { dst: u8, idx: u16 }
    SetGlobal(0x27) { idx: u16, src: u8 }

    // jump
    Jmp(0x28) { off: i16 }
    Jf(0x29) { src: u8, off: i16 }

    // function
    Call(0x2A) { idx: u16, arg_start: u8, ret: u8 }
    Ret(0x2B) { src: u8 }

    // array
    NewArray(0x2C) { dst: u8, size: u32 }
    ArrayLen(0x2D) { dst: u8, src: u8 }
    GetIndex(0x2E) { dst: u8, idx: u32, arr: u8 }
    SetIndex(0x2F) { idx: u32, src: u8, arr: u8 }
}