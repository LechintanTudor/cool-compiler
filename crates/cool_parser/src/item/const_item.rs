use crate::{Expr, ParseTree};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct ConstItem {
    pub expr: Expr,
}

impl ParseTree for ConstItem {
    #[inline]
    fn span(&self) -> Span {
        self.expr.span()
    }
}
