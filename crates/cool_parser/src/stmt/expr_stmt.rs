use crate::expr::Expr;
use crate::ParseTree;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct ExprStmt {
    pub span: Span,
    pub expr: Box<Expr>,
}

impl ParseTree for ExprStmt {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl From<Expr> for ExprStmt {
    fn from(expr: Expr) -> Self {
        assert!(expr.is_promotable_to_stmt());

        Self {
            span: expr.span(),
            expr: Box::new(expr),
        }
    }
}
