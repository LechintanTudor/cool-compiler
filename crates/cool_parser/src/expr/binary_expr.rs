use crate::{BinOp, Expr};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct BinaryExpr {
    pub bin_op: BinOp,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl Section for BinaryExpr {
    #[inline]
    fn span(&self) -> Span {
        self.lhs.span().to(self.rhs.span())
    }
}
