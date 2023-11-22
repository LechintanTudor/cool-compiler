use crate::{Ident, ParseResult, Parser};
use cool_collections::SmallVec;
use cool_lexer::tk;
use cool_span::{Section, Span};

pub type IdentVec = SmallVec<Ident, 2>;

#[derive(Clone, Debug)]
pub struct IdentPath {
    pub idents: IdentVec,
}

impl Section for IdentPath {
    #[inline]
    fn span(&self) -> Span {
        match self.idents.as_slice() {
            [] => Span::EMPTY,
            [first] => first.span,
            [first, .., last] => first.span.to(last.span()),
        }
    }
}

impl Parser<'_> {
    pub fn parse_ident_path(&mut self) -> ParseResult<IdentPath> {
        let mut idents = IdentVec::new();
        idents.push(self.parse_path_ident()?);

        while self.bump_if_eq(tk::dot).is_some() {
            idents.push(self.parse_path_ident()?);
        }

        Ok(IdentPath { idents })
    }
}
