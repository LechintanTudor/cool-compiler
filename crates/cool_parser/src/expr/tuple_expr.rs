use crate::expr::Expr;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct TupleExpr {
    pub span: Span,
    pub elems: Vec<Expr>,
    pub has_trailing_comma: bool,
}

impl Section for TupleExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}
