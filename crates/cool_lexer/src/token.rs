use crate::{Punct, Symbol};
use cool_derive::Section;
use cool_span::Span;
use derive_more::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Section, Debug)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Display)]
pub enum TokenKind {
    #[display("<unknown>")]
    Unknown,

    #[display("{}", _0.as_str())]
    Keyword(Symbol),

    #[display("{}", _0.as_str())]
    Ident(Symbol),

    #[display("{}", _0.as_str())]
    Punct(Punct),

    #[display("<whitespace>")]
    Whitespace,

    #[display("<comment>")]
    Comment,

    #[display("<eof>")]
    Eof,
}
