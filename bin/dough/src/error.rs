macro_rules! error {
    (
        $(
            $variant:ident($inner:path)
        ),* $(,)?
    ) => {
        #[derive(Debug)]
        pub(crate) enum Error {
            $( $variant($inner) ),*
        }

        $(
            impl From<$inner> for Error {
                fn from(error: $inner) -> Self {
                    Self::$variant(error)
                }
            }
        )*
    };
}

error! {
    Io(std::io::Error),
    Lexer(lexer::Error),
    Parser(parser::Error),
    Semantic(semantic::Error),
    Vm(vm::Error),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
