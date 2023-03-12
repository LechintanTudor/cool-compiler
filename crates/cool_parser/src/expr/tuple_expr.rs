use crate::expr::Expr;
use crate::ParseTree;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct TupleExpr {
    pub span: Span,
    pub exprs: Vec<Expr>,
    pub has_trailing_comma: bool,
}

impl ParseTree for TupleExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}
