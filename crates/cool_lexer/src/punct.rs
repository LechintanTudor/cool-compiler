use derive_more::{Display, Error};
use std::fmt;

#[derive(Clone, Error, Display, Debug)]
#[display("Invalid punctuation")]
pub struct InvalidPunct;

macro_rules! define_punct {
    { $($Variant:ident: $display:literal $(from $source:literal)?,)+ } => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub enum Punct {
            $($Variant,)+
        }

        impl Punct {
            #[must_use]
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$Variant => $display,)+
                }
            }
        }

        impl fmt::Display for Punct {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl TryFrom<char> for Punct {
            type Error = InvalidPunct;

            fn try_from(char: char) -> Result<Self, Self::Error> {
                let punctuation = match char {
                    $($($source => Self::$Variant,)?)+
                    _ => return Err(InvalidPunct),
                };

                Ok(punctuation)
            }
        }

        #[allow(non_upper_case_globals)]
        pub(crate) mod punct_tk {
            use crate::{Punct, TokenKind};
            use paste::paste;

            paste! {
                $(
                    pub const [<$Variant:snake:lower>]: TokenKind
                        = TokenKind::Punct(Punct::$Variant);
                )+
            }
        }
    };
}

define_punct! {
    Plus: "+" from '+',
    Minus: "-" from '-',
    Star: "*" from '*',
    Slash: "/" from '/',
    Percent: "%" from '%',
    Caret: "^" from '^',

    Not: "!" from '!',
    And: "&" from '&',
    Or: "|" from '|',
    AndAnd: "&&",
    OrOr: "||",
    Shl: "<<",
    Shr: ">>",

    PlusEq: "+=",
    MinusEq: "-=",
    StarEq: "*=",
    SlashEq: "/=",
    PerecentEq: "%=",
    CaretEq: "^=",
    AndEq: "&=",
    OrEq: "|=",
    ShlEq: "<<=",
    ShrEq: ">>=",

    Eq: "=" from '=',
    EqEq: "==",
    Ne: "!=",
    Gt: ">" from '>',
    Lt: "<" from '<',
    Ge: ">=",
    Le: "<=",

    Arrow: "->",
    FatArrow: "=>",

    Dot: "." from '.',
    DotDot: "..",
    DotDotDot: "...",
    Comma: "," from ',',
    Semicolon: ";" from ';',
    Colon: ":" from ':',

    OpenBrace: "{" from '{',
    CloseBrace: "}" from '}',
    OpenBracket: "[" from '[',
    CloseBracket: "]" from ']',
    OpenParen: "(" from '(',
    CloseParen: ")" from ')',
}

impl Punct {
    pub fn join(&self, other: Self) -> Result<Self, InvalidPunct> {
        let punctuation = match (self, other) {
            // Bitwise
            (Self::And, Self::And) => Self::AndAnd,
            (Self::Or, Self::Or) => Self::OrOr,
            (Self::Lt, Self::Lt) => Self::Shl,
            (Self::Gt, Self::Gt) => Self::Shr,

            // Assignment
            (Self::Plus, Self::Eq) => Self::PlusEq,
            (Self::Minus, Self::Eq) => Self::MinusEq,
            (Self::Star, Self::Eq) => Self::StarEq,
            (Self::Slash, Self::Eq) => Self::SlashEq,
            (Self::Caret, Self::Eq) => Self::CaretEq,
            (Self::And, Self::Eq) => Self::AndEq,
            (Self::Or, Self::Eq) => Self::OrEq,
            (Self::Shl, Self::Eq) => Self::ShlEq,
            (Self::Shr, Self::Eq) => Self::ShrEq,

            // Relational
            (Self::Eq, Self::Eq) => Self::EqEq,
            (Self::Not, Self::Eq) => Self::Ne,
            (Self::Gt, Self::Eq) => Self::Ge,
            (Self::Lt, Self::Eq) => Self::Le,

            // Dots
            (Self::Dot, Self::Dot) => Self::DotDot,
            (Self::DotDot, Self::Dot) => Self::DotDotDot,

            // Arrows
            (Self::Minus, Self::Gt) => Self::Arrow,
            (Self::Eq, Self::Gt) => Self::FatArrow,

            _ => return Err(InvalidPunct),
        };

        Ok(punctuation)
    }
}
