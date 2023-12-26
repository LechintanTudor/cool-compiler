use crate::{ExprId, ParseResult, Parser, StmtId};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct ReturnStmt {
    pub span: Span,
    pub expr_id: Option<ExprId>,
}

impl Parser<'_> {
    pub fn parse_return_stmt(&mut self) -> ParseResult<StmtId> {
        let return_token = self.bump_expect(&tk::kw_return)?;

        let expr_id = (!self.peek().kind.is_punct())
            .then(|| self.parse_expr())
            .transpose()?;

        let span = expr_id.map_or(return_token.span, |expr_id| {
            return_token.span.to(self[expr_id].span())
        });

        Ok(self.add_stmt(ReturnStmt { span, expr_id }))
    }
}
