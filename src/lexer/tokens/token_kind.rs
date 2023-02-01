use crate::lexer::{Literal, Operator, Separator};
use crate::symbol::Symbol;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenKind {
    Unknown,
    Keyword(Symbol),
    Ident(Symbol),
    Literal(Literal),
    Operator(Operator),
    Separator(Separator),
    Whitespace,
    Comment,
    Eof,
}

impl TokenKind {
    pub fn is<T>(&self, token: T) -> bool
    where
        T: Into<TokenKind>,
    {
        self == &token.into()
    }

    pub fn is_lang_part(&self) -> bool {
        !matches!(self, Self::Comment | Self::Whitespace)
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "<unknown>"),
            Self::Keyword(symbol) => fmt::Display::fmt(symbol, f),
            Self::Ident(symbol) => fmt::Display::fmt(symbol, f),
            Self::Literal(literal) => fmt::Display::fmt(literal, f),
            Self::Operator(operator) => write!(f, "{}", operator),
            Self::Separator(separator) => write!(f, "{}", separator),
            Self::Whitespace => write!(f, "<whitespace>"),
            Self::Comment => write!(f, "<comment>"),
            Self::Eof => write!(f, "<eof>"),
        }
    }
}

impl From<Symbol> for TokenKind {
    fn from(symbol: Symbol) -> Self {
        if symbol.is_keyword() {
            Self::Keyword(symbol)
        } else {
            Self::Ident(symbol)
        }
    }
}

impl From<Operator> for TokenKind {
    fn from(operator: Operator) -> Self {
        Self::Operator(operator)
    }
}

impl From<Separator> for TokenKind {
    fn from(separator: Separator) -> Self {
        Self::Separator(separator)
    }
}
