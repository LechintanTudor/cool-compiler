use crate::{BinOp, Expr, ParseTree};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct BinaryExpr {
    pub bin_op: BinOp,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl ParseTree for BinaryExpr {
    #[inline]
    fn span(&self) -> Span {
        self.lhs.span().to(self.rhs.span())
    }
}
