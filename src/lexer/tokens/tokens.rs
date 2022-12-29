use crate::lexer::{Keyword, Operator, Separator};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token {
    Keyword(Keyword),
    Operator(Operator),
    Separator(Separator),
    Wildcard,
    Identifier { index: usize },
    Literal { index: usize },
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
