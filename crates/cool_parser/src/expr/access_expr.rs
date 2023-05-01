use crate::{Expr, Ident, ParseTree};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct AccessExpr {
    pub base: Box<Expr>,
    pub ident: Ident,
}

impl ParseTree for AccessExpr {
    #[inline]
    fn span(&self) -> Span {
        self.base.span().to(self.ident.span())
    }
}
