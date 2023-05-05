use crate::Expr;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ConstItem {
    pub expr: Expr,
}

impl Section for ConstItem {
    #[inline]
    fn span(&self) -> Span {
        self.expr.span()
    }
}
