use crate::{Expr, ParseTree};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct DerefExpr {
    pub span: Span,
    pub base: Box<Expr>,
}

impl ParseTree for DerefExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}
