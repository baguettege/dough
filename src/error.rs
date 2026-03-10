use crate::span::Span;

#[derive(Debug)]
pub enum DoughError {
    Runtime(RuntimeError),
    Compile(CompileError),
    Semantic(SemanticError),
    Parse(ParseError),
    Lex(LexError),
}

#[derive(Debug)]
pub enum RuntimeError {

}

#[derive(Debug)]
pub enum CompileError {
    
}

#[derive(Debug)]
pub enum SemanticError {

}

#[derive(Debug)]
pub enum ParseError {

}

#[derive(Debug)]
pub enum LexError {
    UnexpectedChar(char, Span),
    UnterminatedString(Span),
    InvalidNumber(String, Span)
}