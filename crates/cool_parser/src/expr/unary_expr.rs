use crate::{Expr, ParseResult, Parser, UnaryOp};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Box<Expr>,
}

impl Section for UnaryExpr {
    #[inline]
    fn span(&self) -> Span {
        self.op.span().to(self.expr.span())
    }
}

impl Parser<'_> {
    pub fn parse_unary_expr(&mut self) -> ParseResult<UnaryExpr> {
        Ok(UnaryExpr {
            op: self.parse_unary_op()?,
            expr: Box::new(self.parse_expr()?),
        })
    }
}
