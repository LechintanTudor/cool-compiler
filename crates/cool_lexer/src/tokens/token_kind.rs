use crate::symbols::Symbol;
use crate::tokens::{Literal, LiteralKind, Punctuation};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenKind {
    Unknown,
    Keyword(Symbol),
    Ident(Symbol),
    Literal(Literal),
    Punctuation(Punctuation),
    Whitespace,
    Comment,
    Eof,
}

impl TokenKind {
    pub const fn is_lang_part(&self) -> bool {
        !matches!(self, Self::Comment | Self::Whitespace)
    }
}

pub mod tk {
    pub use crate::tokens::punctuation::tk::*;
    pub use crate::tokens::symbol::tk::*;
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "<unknown>"),
            Self::Keyword(symbol) => fmt::Display::fmt(symbol, f),
            Self::Ident(symbol) => fmt::Display::fmt(symbol, f),
            Self::Literal(literal) => fmt::Display::fmt(literal, f),
            Self::Punctuation(punctuation) => fmt::Display::fmt(punctuation, f),
            Self::Whitespace => write!(f, "<whitespace>"),
            Self::Comment => write!(f, "<comment>"),
            Self::Eof => write!(f, "<eof>"),
        }
    }
}

impl From<Symbol> for TokenKind {
    fn from(symbol: Symbol) -> Self {
        if symbol.is_keyword() {
            if symbol.is_bool_literal() {
                Self::Literal(Literal {
                    kind: LiteralKind::Boolean,
                    symbol,
                })
            } else {
                Self::Keyword(symbol)
            }
        } else {
            Self::Ident(symbol)
        }
    }
}

impl From<Punctuation> for TokenKind {
    fn from(punctuation: Punctuation) -> Self {
        Self::Punctuation(punctuation)
    }
}
