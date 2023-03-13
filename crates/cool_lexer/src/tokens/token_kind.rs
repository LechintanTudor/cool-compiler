use crate::symbols::Symbol;
use crate::tokens::{Group, Literal, LiteralKind, Punctuation};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenKind {
    Unknown,
    Keyword(Symbol),
    Ident(Symbol),
    Literal(Literal),
    Prefix(Symbol),
    Punctuation(Punctuation),
    Whitespace,
    Comment,
    Group(Group),
    Eof,
}

impl TokenKind {
    #[inline]
    pub const fn is_lang_part(&self) -> bool {
        !matches!(self, Self::Whitespace | Self::Comment | Self::Group(_))
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "<unknown>"),
            Self::Keyword(symbol) => fmt::Display::fmt(symbol, f),
            Self::Ident(symbol) => fmt::Display::fmt(symbol, f),
            Self::Literal(literal) => fmt::Display::fmt(literal, f),
            Self::Prefix(symbol) => fmt::Display::fmt(symbol, f),
            Self::Punctuation(punctuation) => fmt::Display::fmt(punctuation, f),
            Self::Whitespace => write!(f, "<whitespace>"),
            Self::Comment => write!(f, "<comment>"),
            Self::Group(group) => fmt::Display::fmt(group, f),
            Self::Eof => write!(f, "<eof>"),
        }
    }
}

impl From<Symbol> for TokenKind {
    fn from(symbol: Symbol) -> Self {
        if symbol.is_keyword() {
            if symbol.is_bool_literal() {
                Self::Literal(Literal {
                    kind: LiteralKind::Bool,
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
    #[inline]
    fn from(punctuation: Punctuation) -> Self {
        Self::Punctuation(punctuation)
    }
}
