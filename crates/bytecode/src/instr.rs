use crate::{Decoder, Idx, Reg, Argc, Off};

macro_rules! instr {
    (
        $(
            $opcode:literal => $mnemonic:ident {
                $(
                    $field:ident: $ty:ty
                ),* $(,)?
            }
        ),+ $(,)?
    ) => {
        pub enum Instr {
            $(
                $mnemonic {
                    $( $field: $ty ),*
                }
            ),*
        }

        impl $crate::Encode for Instr {
            fn encode(&self, encoder: &mut $crate::Encoder) {
                match self {
                    $(
                        Self::$mnemonic { $( $field ),* } => {
                            encoder.encode(&$opcode);
                            $( encoder.encode($field); )*
                        },
                    )*
                }
            }
        }

        impl $crate::Decode for Instr {
            fn decode(decoder: &mut Decoder) -> crate::Result<Self> {
                let opcode = decoder.decode::<$crate::Opcode>()?;

                match opcode {
                    $(
                        $opcode => Ok(Self::$mnemonic {
                            $(
                                $field: decoder.decode::<$ty>()?
                            ),*
                        }),
                    )*
                    _ => Err($crate::Error::UnknownOpcode(opcode)),
                }
            }
        }
    };
}

instr! {
    0x00 => Nop {},
    0x01 => Halt {},

    0x02 => IAdd { dst: Reg, lhs: Reg, rhs: Reg },
    0x03 => ISub { dst: Reg, lhs: Reg, rhs: Reg },
    0x04 => IMul { dst: Reg, lhs: Reg, rhs: Reg },
    0x05 => IDiv { dst: Reg, lhs: Reg, rhs: Reg },
    0x06 => INeg { dst: Reg, src: Reg },
    0x07 => IEq { dst: Reg, lhs: Reg, rhs: Reg },
    0x08 => INe { dst: Reg, lhs: Reg, rhs: Reg },
    0x09 => ILt { dst: Reg, lhs: Reg, rhs: Reg },
    0x0A => ILe { dst: Reg, lhs: Reg, rhs: Reg },
    0x0B => IGt { dst: Reg, lhs: Reg, rhs: Reg },
    0x0C => IGe { dst: Reg, lhs: Reg, rhs: Reg },

    0x0D => FAdd { dst: Reg, lhs: Reg, rhs: Reg },
    0x0E => FSub { dst: Reg, lhs: Reg, rhs: Reg },
    0x0F => FMul { dst: Reg, lhs: Reg, rhs: Reg },
    0x10 => FDiv { dst: Reg, lhs: Reg, rhs: Reg },
    0x11 => FNeg { dst: Reg, src: Reg },
    0x12 => FEq { dst: Reg, lhs: Reg, rhs: Reg },
    0x13 => FNe { dst: Reg, lhs: Reg, rhs: Reg },
    0x14 => FLt { dst: Reg, lhs: Reg, rhs: Reg },
    0x15 => FLe { dst: Reg, lhs: Reg, rhs: Reg },
    0x16 => FGt { dst: Reg, lhs: Reg, rhs: Reg },
    0x17 => FGe { dst: Reg, lhs: Reg, rhs: Reg },

    0x18 => BAnd { dst: Reg, lhs: Reg, rhs: Reg },
    0x19 => BOr { dst: Reg, lhs: Reg, rhs: Reg },
    0x1A => BNot { dst: Reg, src: Reg },

    0x1B => SAdd { dst: Reg, lhs: Reg, rhs: Reg },
    0x1C => SEq { dst: Reg, lhs: Reg, rhs: Reg },
    0x1D => SNe { dst: Reg, lhs: Reg, rhs: Reg },

    0x1E => Mov { dst: Reg, src: Reg },
    0x1F => Ldc { dst: Reg, idx: Idx },
    0x20 => Ldu { dst: Reg },
    0x21 => Ldg { dst: Reg, idx: Idx },
    0x22 => Stg { idx: Idx, src: Reg },

    0x23 => Call { dst: Reg, idx: Idx, argc: Argc },
    0x24 => Ret { src: Reg },

    0x25 => Jmp { off: Off },
    0x26 => Jf { dst: Reg, off: Off },
}
