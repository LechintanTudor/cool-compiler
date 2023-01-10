use crate::lexer::{Keyword, Operator, Separator};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token {
    Keyword(Keyword),
    Operator(Operator),
    Separator(Separator),
    Underscore,
    Ident { index: u32 },
    Literal { index: u32 },
    Eof,
}

impl Token {
    pub fn is<T>(&self, token: T) -> bool
    where
        T: Into<Token>,
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

impl From<Keyword> for Token {
    fn from(keyword: Keyword) -> Self {
        Self::Keyword(keyword)
    }
}

impl From<Operator> for Token {
    fn from(operator: Operator) -> Self {
        Self::Operator(operator)
    }
}

impl From<Separator> for Token {
    fn from(separator: Separator) -> Self {
        Self::Separator(separator)
    }
}
