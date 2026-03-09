pub enum DoughError {
    RuntimeError(RuntimeError),
    CompileError(CompileError)
}

pub enum RuntimeError {
    TypeError {
        expected: &'static str,
        got: &'static str,
    },

    NotCallable,
    ArityMismatch {
        expected: u8,
        got: u8
    },

    UndefinedVariable(String),

    IndexOutOfBounds {
        len: usize,
        index: usize
    },

    DivisionByZero,

    StackOverflow
}

pub enum CompileError {

}