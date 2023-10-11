use crate::Symbol;
use cool_derive::Section;
use cool_span::Span;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Section, Debug)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TokenKind {
    Unknown,
    Keyword(Symbol),
    Ident(Symbol),
    Whitespace,
    Eof,
}
