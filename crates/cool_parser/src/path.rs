use crate::{Ident, ParseResult, ParseTree, Parser};
use cool_lexer::tokens::{tk, Token};
use cool_span::Span;
use smallvec::SmallVec;

pub type ItemPathVec = SmallVec<[Ident; 2]>;

#[derive(Clone, Debug)]
pub struct SymbolPath {
    pub idents: ItemPathVec,
}

impl ParseTree for SymbolPath {
    fn span(&self) -> Span {
        match (self.idents.first(), self.idents.last()) {
            (Some(first), Some(last)) => first.span_to(last),
            _ => Span::empty(),
        }
    }
}
impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_path(&mut self) -> ParseResult<SymbolPath> {
        let mut idents = ItemPathVec::new();
        idents.push(self.parse_ident()?);

        while self.peek().kind == tk::DOT {
            self.bump_expect(&[tk::DOT])?;
            idents.push(self.parse_ident()?);
        }

        Ok(SymbolPath { idents })
    }
}
