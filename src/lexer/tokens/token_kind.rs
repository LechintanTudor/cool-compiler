use crate::lexer::{Keyword, Operator, Separator};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenKind {
    Unknown,
    Keyword(Keyword),
    Operator(Operator),
    Separator(Separator),
    Underscore,
    Ident { index: u32 },
    Literal { index: u32 },
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

    pub fn as_ident_index(&self) -> Option<u32> {
        match self {
            Self::Ident { index } => Some(*index),
            _ => None,
        }
    }

    pub fn as_literal_index(&self) -> Option<u32> {
        match self {
            Self::Literal { index } => Some(*index),
            _ => None,
        }
    }

    pub fn is_lang_part(&self) -> bool {
        !matches!(self, Self::Comment | Self::Whitespace)
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "<unknown>"),
            Self::Keyword(keyword) => write!(f, "{}", keyword),
            Self::Operator(operator) => write!(f, "{}", operator),
            Self::Separator(separator) => write!(f, "{}", separator),
            Self::Underscore => write!(f, "_"),
            Self::Ident { index } => write!(f, "<ident {}>", index),
            Self::Literal { index } => write!(f, "<literal {}>", index),
            Self::Whitespace => write!(f, "<whitespace>"),
            Self::Comment => write!(f, "<comment>"),
            Self::Eof => write!(f, "<eof>"),
        }
    }
}

impl From<Keyword> for TokenKind {
    fn from(keyword: Keyword) -> Self {
        Self::Keyword(keyword)
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
