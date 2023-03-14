use crate::{Ident, ParseResult, ParseTree, Parser};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct IdentExpr {
    pub ident: Ident,
}

impl ParseTree for IdentExpr {
    #[inline]
    fn span(&self) -> Span {
        self.ident.span
    }
}

impl Parser<'_> {
    pub fn parse_ident_expr(&mut self) -> ParseResult<IdentExpr> {
        let ident = self.parse_path_ident()?;

        Ok(IdentExpr { ident })
    }
}
