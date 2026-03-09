use std::fmt::{Display, Formatter};
use crate::lexer::cursor::Cursor;
use crate::source::SourceRange;

#[derive(Debug)]
pub(crate) struct Token {
    kind: TokenKind,
    span: SourceRange
}

impl Token {
    pub(crate) fn new(kind: TokenKind, span: SourceRange) -> Self {
        Self { kind, span }
    }

    pub(crate) fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub(crate) fn span(&self) -> SourceRange {
        self.span
    }
}

macro_rules! keywords {
    ( $( $kw:literal => $variant:ident ),* $(,)? ) => {
        pub(crate) fn from_keyword(s: &str) -> Option<TokenKind> {
            match s {
                $( $kw => Some(TokenKind::$variant), )*
                _ => None
            }
        }
    };
}

macro_rules! puncts {
    (
        one {
            $( $p1variant:ident ($c1:literal) ),* $(,)?
        }
        two {
            $( $p2variant:ident ($c2a:literal, $c2b:literal) ),* $(,)?
        }
    ) => {
        fn from_one_char_punct(cursor: &mut Cursor) -> Option<TokenKind> {
            match cursor.peek() {
                $( $c1 => {
                    cursor.skip(1);
                    Some(TokenKind::$p1variant)
                }, )*
                _ => None
            }
        }

        fn from_two_char_punct(cursor: &mut Cursor) -> Option<TokenKind> {
            match (cursor.peek(), cursor.peek_next()) {
                $( ($c2a, $c2b) => {
                    cursor.skip(2);
                    Some(TokenKind::$p2variant)
                }, )*
                _ => None
            }
        }

        pub(crate) fn from_punct(cursor: &mut Cursor) -> Option<TokenKind> {
            Self::from_two_char_punct(cursor)
            .or_else(|| Self::from_one_char_punct(cursor))
        }
    };
}

macro_rules! define_token_kind {
    (
        keywords {
            $( $kwvariant:ident ($kwstr:literal) ),* $(,)?
        }
        puncts {
            one {
                $( $p1variant:ident ($c1:literal) ),* $(,)?
            }
            two {
                $( $p2variant:ident ($c2a:literal, $c2b:literal) ),* $(,)?
            }
        }
        values {
            $( $vvariant:ident ($ty:ty) ),* $(,)?
        }
    ) => {
        #[derive(Debug, PartialEq)]
        pub(crate) enum TokenKind {
            $( $kwvariant, )*
            $( $p1variant, )*
            $( $p2variant, )*
            $( $vvariant($ty), )*
            Eof,
        }

        impl TokenKind {
            keywords! { $( $kwstr => $kwvariant ),* }
            puncts! {
                one { $( $p1variant($c1), )* }
                two { $( $p2variant($c2a, $c2b), )* }
            }
        }

        impl Display for TokenKind {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                match self {
                    $( TokenKind::$kwvariant => write!(f, $kwstr), )*
                    $( TokenKind::$p1variant => write!(f, "{}", $c1), )*
                    $( TokenKind::$p2variant => write!(f, "{}{}", $c2a, $c2b), )*
                    $( TokenKind::$vvariant(val) => write!(f, "{:?}", val), )*
                    TokenKind::Eof => write!(f, "<EOF>"),
                }
            }
        }
    };
}

define_token_kind! {
    keywords {
        Var("var"),
        Func("func"),

        Int("int"),
        Float("float"),
        Bool("bool"),
        Str("str"),

        If("if"),
        Else("else"),
        While("while"),
        Return("return"),

        And("and"),
        Or("or"),
        Not("not"),
    }

    puncts {
        one {
            Assign('='),

            Plus('+'),
            Minus('-'),
            Star('*'),
            Slash('/'),

            Lt('<'),
            Gt('>'),

            LParen('('),
            RParen(')'),
            LBrace('{'),
            RBrace('}'),
            LBrack('['),
            RBrack(']'),
            Colon(':'),
            Semicolon(';'),
            Comma(','),
        }

        two {
            Eq('=', '='),
            Ne('!', '='),
            Le('<', '='),
            Ge('>', '='),
        }
    }

    values {
        Ident(String),

        IntLit(i64),
        FloatLit(f64),
        BoolLit(bool),
        StrLit(String),
    }
}