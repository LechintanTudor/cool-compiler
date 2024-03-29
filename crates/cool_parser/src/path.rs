use crate::{Ident, ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};
use smallvec::SmallVec;

pub type IdentVec = SmallVec<[Ident; 2]>;

#[derive(Clone, Debug)]
pub struct IdentPath {
    pub idents: IdentVec,
}

impl Section for IdentPath {
    fn span(&self) -> Span {
        match (self.idents.first(), self.idents.last()) {
            (Some(first), Some(last)) => first.span().to(last.span()),
            _ => Span::empty(),
        }
    }
}

impl Parser<'_> {
    pub fn parse_access_path(&mut self) -> ParseResult<IdentPath> {
        let mut idents = IdentVec::new();
        idents.push(self.parse_access_path_ident()?);

        while self.bump_if_eq(tk::DOT).is_some() {
            idents.push(self.parse_access_path_ident()?);
        }

        Ok(IdentPath { idents })
    }

    pub fn parse_import_path(&mut self) -> ParseResult<IdentPath> {
        let mut idents = IdentVec::new();
        idents.push(self.parse_path_ident()?);

        while self.peek().kind == tk::DOT {
            self.bump_expect(&tk::DOT)?;
            idents.push(self.parse_path_ident()?);
        }

        Ok(IdentPath { idents })
    }
}
