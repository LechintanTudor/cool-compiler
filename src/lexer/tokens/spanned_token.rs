use crate::lexer::{Separator, Token};
use crate::utils::Span;

#[derive(Clone, Copy, Debug)]
pub struct SpannedToken {
    pub span: Span,
    pub token: Token,
}

impl SpannedToken {
    pub fn new<T>(start: u32, len: u32, token: T) -> Self
    where
        T: Into<Token>,
    {
        Self {
            span: Span { start, len },
            token: token.into(),
        }
    }

    pub fn separator(start: u32, separator: Separator) -> Self {
        Self {
            span: Span { start, len: 1 },
            token: separator.into(),
        }
    }
}
