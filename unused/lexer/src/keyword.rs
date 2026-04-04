use crate::token::Token;

impl TryFrom<&str> for Token {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "var" => Ok(Token::Var),
            "func" => Ok(Token::Func),
            "if" => Ok(Token::If),
            "else" => Ok(Token::Else),
            "while" => Ok(Token::While),
            "return" => Ok(Token::Return),
            "not" => Ok(Token::Not),
            "and" => Ok(Token::And),
            "or" => Ok(Token::Or),
            _ => Err(()),
        }
    }
}
