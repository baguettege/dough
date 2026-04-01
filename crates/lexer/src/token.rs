pub enum Token {
    // identifier
    Ident(String),

    // literals
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),

    // keywords
    Var,
    Func,
    If,
    Else,
    While,
    Return,
    Not,
    And,
    Or,

    // punctuation
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBrack,
    RBrack,
    Colon,
    Semicolon,
    Comma,
    Dot,

    // operators
    Plus,
    Minus,
    Star,
    Slash,
    EqEq,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    // special
    Eof,
}


