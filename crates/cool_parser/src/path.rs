use crate::{Ident, ParseResult, ParseTree, Parser};
use cool_lexer::symbols::sym;
use cool_lexer::tokens::{tk, Token};
use cool_span::Span;
use smallvec::SmallVec;

pub type IdentVec = SmallVec<[Ident; 2]>;

#[derive(Clone, Debug)]
pub struct SymbolPath {
    pub idents: IdentVec,
}

impl SymbolPath {
    pub fn ends_with_glob(&self) -> bool {
        self.idents
            .last()
            .filter(|ident| ident.symbol == sym::GLOB)
            .is_some()
    }

    pub fn is_valid_import(&self) -> bool {
        // TODO: Implement the function
        // enum SymbolPathParseState {
        //     Initial,
        //     Crate,
        //     SelfOrSuper,
        //     Ident,
        //     Final,
        // }

        true
    }
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
    pub fn parse_import_path(&mut self) -> ParseResult<SymbolPath> {
        let mut idents = IdentVec::new();
        idents.push(self.parse_path_ident()?);

        while self.peek().kind == tk::DOT {
            self.bump_expect(&[tk::DOT])?;
            idents.push(self.parse_path_ident()?);
        }

        Ok(SymbolPath { idents })
    }
}
