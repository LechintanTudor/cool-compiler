use crate::expr::Expr;
use crate::ParseTree;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct ExprStmt {
    pub span: Span,
    pub expr: Expr,
}

impl ParseTree for ExprStmt {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}
