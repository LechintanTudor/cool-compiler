use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct InvalidPunctuation;

impl Error for InvalidPunctuation {}

impl fmt::Display for InvalidPunctuation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid punctuation")
    }
}

macro_rules! Punctuation {
    { $($variant:ident: $display:literal $(from $source:literal)?,)+ } => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Punctuation {
            $($variant,)+
        }

        impl fmt::Display for Punctuation {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let display_str = match self {
                    $(Self::$variant => $display,)+
                };

                write!(f, "{}", display_str)
            }
        }

        impl TryFrom<char> for Punctuation {
            type Error = InvalidPunctuation;

            fn try_from(char: char) -> Result<Self, Self::Error> {
                let punctuation = match char {
                    $($($source => Self::$variant,)?)+
                    _ => return Err(InvalidPunctuation),
                };

                Ok(punctuation)
            }
        }

        #[allow(dead_code)]
        pub mod tk {
            use crate::tokens::{Punctuation, TokenKind};
            use paste::paste;

            paste! {
                $(
                    pub const [<$variant:snake:upper>]: TokenKind
                        = TokenKind::Punctuation(Punctuation::$variant);
                )+
            }
        }
    };
}

Punctuation! {
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

impl Punctuation {
    pub fn join(&self, other: Self) -> Result<Self, InvalidPunctuation> {
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

            // Arrows
            (Self::Minus, Self::Gt) => Self::Arrow,
            (Self::Eq, Self::Gt) => Self::FatArrow,

            _ => return Err(InvalidPunctuation),
        };

        Ok(punctuation)
    }
}
