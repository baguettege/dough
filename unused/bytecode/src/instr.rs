macro_rules! define_instrs {
    (
        $(
            $mnemonic:ident = $opcode:literal {
                $(
                    $operand:ident: $ty:ty
                ),* $(,)?
            }
        ),* $(,)?
    ) => {
        #[derive(Debug)]
        pub enum Instr {
            $(
                $mnemonic {
                    $( $operand: $ty ),*
                }
            ),*
        }

        impl std::fmt::Display for Instr {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(
                        Self::$mnemonic { $( $operand ),* } => {
                            write!(f, "{}", stringify!($mnemonic))?;
                            $( write!(f, "{}:{}", stringify!($operand), $operand)?; )*
                        }
                    )*
                }

                Ok(())
            }
        }
    };
}

define_instrs! {
    Nop = 0x00 {},
}
