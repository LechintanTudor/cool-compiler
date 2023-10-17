use crate::{Literal, Punct, Symbol};
use cool_derive::Section;
use cool_span::Span;
use derive_more::Display;

#[derive(Clone, Copy, Section, Debug)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

#[derive(Clone, Copy, Debug, Display)]
pub enum TokenKind {
    #[display("<unknown>")]
    Unknown,

    #[display("{}", _0.as_str())]
    Keyword(Symbol),

    #[display("{}", _0.as_str())]
    Ident(Symbol),

    Literal(Literal),

    #[display("{}", _0.as_str())]
    Punct(Punct),

    #[display("<whitespace>")]
    Whitespace,

    #[display("<comment>")]
    Comment,

    #[display("<eof>")]
    Eof,
}

impl From<Literal> for TokenKind {
    #[inline]
    fn from(literal: Literal) -> Self {
        Self::Literal(literal)
    }
}

impl From<Punct> for TokenKind {
    #[inline]
    fn from(punct: Punct) -> Self {
        Self::Punct(punct)
    }
}
