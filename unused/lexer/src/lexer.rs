use crate::Token;
use crate::error::Result;

pub struct Lexer<'a> {
    _placeholder_change_this_later: std::marker::PhantomData<&'a str>
}

impl<'a> Lexer<'a> {
    pub fn tokenize(source: &'a str) -> Result<Vec<Token>> {
        todo!()
    }
}
