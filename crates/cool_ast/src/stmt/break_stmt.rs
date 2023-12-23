use crate::{ExprId, ParseResult, Parser, StmtId};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct BreakStmt {
    pub span: Span,
    pub expr_id: Option<ExprId>,
}

impl Parser<'_> {
    pub fn parse_break_stmt(&mut self) -> ParseResult<StmtId> {
        let break_token = self.bump_expect(&tk::kw_break)?;

        let expr_id = (!self.peek().kind.is_punct())
            .then(|| self.parse_expr())
            .transpose()?;

        let span = expr_id
            .map(|expr_id| break_token.span.to(self[expr_id].span()))
            .unwrap_or(break_token.span);

        Ok(self.add_stmt(BreakStmt { span, expr_id }))
    }
}
