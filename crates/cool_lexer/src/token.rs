use cool_derive::Section;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

#[derive(Clone, Debug)]
pub enum TokenKind {
    // Empty
}
