package io.github.bagu.dough.token;

public enum TokenKind {
    STR,
    INT,
    FLOAT,
    BOOL,
    VOID,

    STR_LIT,
    INT_LIT,
    FLOAT_LIT,
    TRUE,
    FALSE,

    IDENT,
    ASSIGN,

    VAR,
    FUNC,

    IF,
    ELSE,
    WHILE,
    RETURN,

    PLUS,
    MINUS,
    SLASH,
    STAR,

    EQ,
    NEQ,
    GT,
    LT,
    LE,
    GE,

    AND,
    OR,
    NOT,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    COLON,
    COMMA,
    SEMICOLON,

    EOF;
}
