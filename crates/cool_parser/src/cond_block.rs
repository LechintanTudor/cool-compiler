use crate::{BlockExpr, Expr, ParseResult, Parser};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct CondBlock {
    pub cond: Expr,
    pub expr: BlockExpr,
}

impl Section for CondBlock {
    #[inline]
    fn span(&self) -> Span {
        self.cond.span().to(self.expr.span())
    }
}

impl Parser<'_> {
    pub fn parse_cond_block(&mut self) -> ParseResult<CondBlock> {
        Ok(CondBlock {
            cond: self.parse_guard_expr()?,
            expr: self.parse_block_expr()?,
        })
    }
}
