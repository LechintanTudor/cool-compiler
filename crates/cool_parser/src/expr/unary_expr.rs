use crate::{ExprId, ParseResult, Parser, UnaryOp};
use cool_derive::Section;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct UnaryExpr {
    pub span: Span,
    pub op: UnaryOp,
    pub expr: ExprId,
}

impl Parser<'_> {
    pub fn parse_unary_expr(&mut self) -> ParseResult<ExprId> {
        let start_span = self.peek().span;
        let op = self.parse_unary_op()?;

        let expr = self.parse_expr()?;
        let end_span = self[expr].span();

        Ok(self.add_expr(UnaryExpr {
            span: start_span.to(end_span),
            op,
            expr,
        }))
    }
}
