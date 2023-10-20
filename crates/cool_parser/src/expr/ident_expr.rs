use crate::{Ident, ParseResult, Parser};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct IdentExpr {
    pub ident: Ident,
}

impl Section for IdentExpr {
    #[inline]
    fn span(&self) -> Span {
        self.ident.span
    }
}

impl Parser<'_> {
    #[inline]
    pub fn parse_ident_expr(&mut self) -> ParseResult<IdentExpr> {
        self.parse_ident().map(|ident| IdentExpr { ident })
    }
}
