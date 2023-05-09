use crate::Expr;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct DerefExpr {
    pub span: Span,
    pub expr: Box<Expr>,
}

impl Section for DerefExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}
