use crate::lexer::{Keyword, Operator, Separator};

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

    pub fn as_lit_index(&self) -> Option<u32> {
        match self {
            Self::Literal { index } => Some(*index),
            _ => None,
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
