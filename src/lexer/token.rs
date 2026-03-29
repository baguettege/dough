use crate::span::{Span, Spanned};

/// A token kind produced by the lexer.
pub(crate) enum TokenKind {
    // literals
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),

    // keywords
    Var,
    Func,
    Return,
    If,
    Else,
    While,
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
    Le,
    Lt,
    Ge,
    Gt,

    // identifiers
    Ident(String),

    // special
    Eof,
}

/// A token with its kind and location in the source text.
pub(crate) struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub(crate) fn kind(&self) -> &TokenKind {
        &self.kind
    }
}

impl Spanned for Token {
    fn span(&self) -> Span {
        self.span
    }
}
