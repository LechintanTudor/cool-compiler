use crate::{ExprId, Parser, StmtId};
use cool_derive::Section;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct ExprStmt {
    pub span: Span,
    pub expr_id: ExprId,
}

impl Parser<'_> {
    pub fn continue_parse_expr_stmt(&mut self, expr_id: ExprId) -> StmtId {
        let span = self[expr_id].span();
        self.add_stmt(ExprStmt { span, expr_id })
    }
}
