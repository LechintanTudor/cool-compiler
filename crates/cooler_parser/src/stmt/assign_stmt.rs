use crate::{AssignOp, ExprId, ParseResult, Parser, StmtId};
use cool_derive::Section;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct AssignStmt {
    pub span: Span,
    pub lhs: ExprId,
    pub op: AssignOp,
    pub rhs: ExprId,
}

impl Parser<'_> {
    pub fn continue_parse_assign_stmt(&mut self, lhs: ExprId) -> ParseResult<StmtId> {
        let op = AssignOp::try_from(self.bump().kind).unwrap();
        let rhs = self.parse_expr()?;

        let start_span = self[lhs].span();
        let end_span = self[rhs].span();

        Ok(self.add_stmt(AssignStmt {
            span: start_span.to(end_span),
            lhs,
            op,
            rhs,
        }))
    }
}
