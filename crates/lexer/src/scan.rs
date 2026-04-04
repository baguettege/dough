use crate::cursor::Cursor;
use crate::Token;

pub(crate) fn from_keyword(keyword: &str) -> Option<Token> {
    match keyword {
        "fn" => Some(Token::Fn),
        "let" => Some(Token::Let),
        "if" => Some(Token::If),
        "else" => Some(Token::Else),
        "while" => Some(Token::While),
        "return" => Some(Token::Return),
        "and" => Some(Token::And),
        "or" => Some(Token::Or),
        "not" => Some(Token::Not),
        _ => None,
    }
}

macro_rules! scan_punct {
    (
        $cursor:expr,
        $(
            $pat:pat => $token:expr
        ),* $(,)?
    ) => {
        match $cursor.peek() {
            $(
                Some($pat) => {
                    $cursor.advance();
                    Some($token)
                },
            )*

            _ => None,
        }
    };
}

pub(crate) fn from_punct(cursor: &mut Cursor) -> Option<Token> {
    scan_punct! {
        cursor,

        '+' => Token::Plus,
        '-' => Token::Minus,
        '*' => Token::Star,
        '/' => Token::Slash,
        '(' => Token::LParen,
        ')' => Token::RParen,
        '{' => Token::LBrace,
        '}' => Token::RBrace,
        ',' => Token::Comma,
        ';' => Token::Semicolon,
    }
}

macro_rules! scan_op {
    (
        $cursor:expr,
        $(
            $first:literal => $token1:expr $(
                , $second:literal => $token2:expr
            )?;
        )*
    ) => {
        match $cursor.peek()? {
            $(
                $first => {
                    $cursor.advance();

                    $(
                        if $cursor.peek() == Some($second) {
                            $cursor.advance();
                            Some($token2)
                        } else
                    )?

                    { $token1 }
                }
            )*

            _ => None,
        }
    };
}

pub(crate) fn from_op(cursor: &mut Cursor) -> Option<Token> {
    scan_op! {
        cursor,

        '=' => Some(Token::Assign), '=' => Token::Eq;
        '<' => Some(Token::Lt), '=' => Token::Le;
        '>' => Some(Token::Gt), '=' => Token::Ge;
        '!' => None, '=' => Token::Ne;
    }
}