use crate::{Idx, Argc, Off};

macro_rules! instr {
    (
        $(
            $opcode:literal => $mnemonic:ident
            $(
                (
                    $( $field:ident: $ty:ty ),* $(,)?
                )
            )?
        ),+ $(,)?
    ) => {
        #[derive(Debug)]
        pub enum Instr {
            $(
                $mnemonic
                $(( $($ty),* ))?
            ),+
        }

        impl Instr {
            #[allow(unused_variables)]
            pub fn size(&self) -> usize {
                size_of::<$crate::Opcode>() + match self {
                    $(
                        Self::$mnemonic $(( $($field),* ))? => {
                            0 $($( +  size_of::<$ty>() )*)?
                        }
                    ),+
                }
            }
        }

        impl $crate::Encode for Instr {
            fn encode(&self, encoder: &mut $crate::Encoder) {
                match self {
                    $(
                        Self::$mnemonic $(( $($field),* ))? => {
                            encoder.encode(&($opcode as $crate::Opcode));
                            $( $( encoder.encode($field); )* )?
                        }
                    ),+
                }
            }
        }

        impl $crate::Decode for Instr {
            fn decode(decoder: &mut $crate::Decoder) -> $crate::Result<Self> {
                let opcode = decoder.decode::<$crate::Opcode>()?;

                match opcode {
                    $(
                        $opcode => Ok(
                            Self::$mnemonic
                            $(( $( decoder.decode::<$ty>()? ),* ))?
                        ),
                    )+
                    _ => Err($crate::Error::UnknownOpcode(opcode)),
                }
            }
        }

        impl std::fmt::Display for Instr {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(
                        Self::$mnemonic $(( $($field),* ))? => {
                            write!(f, "{}", stringify!($mnemonic))?;
                            $($( write!(f, " {}={}", stringify!($field), $field)?; )*)?
                        }
                    ),+
                }
                Ok(())
            }
        }
    };
}

instr! {
    0x00 => Nop,
    0x01 => Halt,

    0x02 => IAdd,
    0x03 => ISub,
    0x04 => IMul,
    0x05 => IDiv,
    0x06 => INeg,
    0x07 => IEq,
    0x08 => INe,
    0x09 => ILt,
    0x0A => ILe,
    0x0B => IGt,
    0x0C => IGe,

    0x0D => FAdd,
    0x0E => FSub,
    0x0F => FMul,
    0x10 => FDiv,
    0x11 => FNeg,
    0x12 => FEq,
    0x13 => FNe,
    0x14 => FLt,
    0x15 => FLe,
    0x16 => FGt,
    0x17 => FGe,

    0x18 => BAnd,
    0x19 => BOr,
    0x1A => BNot,
    0x1B => BEq,
    0x1C => BNe,

    0x1D => SAdd,
    0x1E => SEq,
    0x1F => SNe,

    0x20 => Jmp(off: Off),
    0x21 => Jf(off: Off),

    0x22 => Call(idx: Idx, argc: Argc),
    0x23 => Ret,

    0x24 => Push(idx: Idx),
    0x25 => PushU,
    0x26 => Pop,
    0x27 => Ldl(idx: Idx),
    0x28 => Stl(idx: Idx),
}
