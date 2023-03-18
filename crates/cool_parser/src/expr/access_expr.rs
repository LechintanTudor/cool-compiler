use crate::{Expr, Ident, ParseResult, ParseTree, Parser};
use cool_lexer::tokens::tk;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct AccessExpr {
    pub base: Box<Expr>,
    pub ident: Ident,
}

impl ParseTree for AccessExpr {
    #[inline]
    fn span(&self) -> Span {
        self.base.span_to(&self.ident)
    }
}

impl Parser<'_> {
    pub fn continue_parse_access_expr(&mut self, base: Box<Expr>) -> ParseResult<AccessExpr> {
        self.bump_expect(&tk::DOT)?;
        let ident = self.parse_ident()?;

        Ok(AccessExpr { base, ident })
    }
}
