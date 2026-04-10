#[derive(Debug, Clone)]
pub enum Token {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),

    Ident(String),

    Func,
    Let,
    If,
    Else,
    While,
    Return,

    Plus,
    Minus,
    Star,
    Slash,

    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    And,
    Or,
    Not,

    Assign,

    LParen,
    RParen,
    LBrace,
    RBrace,
    Colon,
    Semicolon,
    Comma,

    Eof,
}
