use crate::expr::Expr;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ParenExpr {
    pub span: Span,
    pub inner: Box<Expr>,
}

impl Section for ParenExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}
