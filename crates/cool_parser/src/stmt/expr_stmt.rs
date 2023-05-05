use crate::expr::Expr;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ExprStmt {
    pub span: Span,
    pub expr: Box<Expr>,
}

impl Section for ExprStmt {
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
