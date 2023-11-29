use crate::{Literal, Punct, Symbol};
use cool_derive::Section;
use cool_span::Span;
use derive_more::Display;

#[derive(Clone, Copy, Section, Debug)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Display)]
pub enum TokenKind {
    #[display("<unknown>")]
    Unknown,

    Keyword(Symbol),

    Ident(Symbol),

    Literal(Literal),

    Punct(Punct),

    #[display("<whitespace>")]
    Whitespace,

    #[display("<comment>")]
    Comment,

    #[display("<eof>")]
    Eof,
}

impl TokenKind {
    #[inline]
    #[must_use]
    pub fn as_literal(self) -> Option<Literal> {
        match self {
            Self::Literal(literal) => Some(literal),
            _ => None,
        }
    }

    #[inline]
    #[must_use]
    pub fn is_lang_part(&self) -> bool {
        !matches!(self, Self::Whitespace | Self::Comment)
    }

    #[inline]
    #[must_use]
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::Literal(_))
    }

    #[inline]
    #[must_use]
    pub fn is_punct(&self) -> bool {
        matches!(self, Self::Punct(_))
    }
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
