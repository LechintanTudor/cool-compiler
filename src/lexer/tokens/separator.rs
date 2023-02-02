use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct InvalidSeparator;

impl Error for InvalidSeparator {}

impl fmt::Display for InvalidSeparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid separator")
    }
}

macro_rules! Separator {
    { $($variant:ident => $char:literal,)+ } => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub enum Separator {
            $($variant,)+
        }

        impl fmt::Display for Separator {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let char = match self {
                    $(Self::$variant => $char,)+
                };

                write!(f, "{}", char)
            }
        }

        impl TryFrom<char> for Separator {
            type Error = InvalidSeparator;

            fn try_from(char: char) -> Result<Self, Self::Error> {
                Ok(match char {
                    $($char => Self::$variant,)+
                    _ => return Err(InvalidSeparator),
                })
            }
        }

        pub mod sep {
            use crate::lexer::{Separator, TokenKind};
            use paste::paste;

            pub const ALL: &[char] = &[$($char,)+];

            paste! {
                $(
                    pub const [<$variant:snake:upper>]: TokenKind
                        = TokenKind::Separator(Separator::$variant);
                )+
            }
        }
    };
}

Separator! {
    Comma => ',',
    Colon => ':',
    Semi => ';',
    OpenParen => '(',
    ClosedParen => ')',
    OpenBracket => '[',
    ClosedBracket => ']',
    OpenBrace => '{',
    ClosedBrace => '}',
}
