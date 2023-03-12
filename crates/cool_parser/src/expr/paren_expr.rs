use crate::expr::Expr;
use crate::ParseTree;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct ParenExpr {
    pub span: Span,
    pub expr: Box<Expr>,
}

impl ParseTree for ParenExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}
