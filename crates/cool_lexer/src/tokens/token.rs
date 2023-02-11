use crate::tokens::TokenKind;
use cool_span::Span;

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

impl Token {
    pub fn eof(source_len: u32) -> Self {
        Self {
            span: Span::new(source_len, 0),
            kind: TokenKind::Eof,
        }
    }
}
