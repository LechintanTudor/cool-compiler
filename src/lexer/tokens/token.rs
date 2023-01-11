use crate::lexer::{Operator, Separator, TokenKind};
use crate::utils::Span;

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

impl Token {
    pub fn new<K>(start: u32, len: u32, kind: K) -> Self
    where
        K: Into<TokenKind>,
    {
        Self {
            span: Span::new(start, len),
            kind: kind.into(),
        }
    }

    pub fn separator(start: u32, separator: Separator) -> Self {
        Self {
            span: Span::new(start, 1),
            kind: separator.into(),
        }
    }

    pub fn operator(start: u32, operator: Operator) -> Self {
        Self {
            span: Span::new(start, operator.len()),
            kind: operator.into(),
        }
    }

    pub fn unknown(start: u32) -> Self {
        Self {
            span: Span::new(start, 1),
            kind: TokenKind::Unknown,
        }
    }

    pub fn eof(source_len: u32) -> Self {
        Self {
            span: Span::new(source_len, 0),
            kind: TokenKind::Eof,
        }
    }
}
