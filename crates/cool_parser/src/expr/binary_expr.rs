use crate::{BinaryOp, Expr};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct BinaryExpr {
    pub op: BinaryOp,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl Section for BinaryExpr {
    #[inline]
    #[must_use]
    fn span(&self) -> Span {
        self.lhs.span().to(self.rhs.span())
    }
}
