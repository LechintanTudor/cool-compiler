use crate::{ExprOrStmt, ParseResult, Parser, StmtId};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct DeferStmt {
    pub span: Span,
    pub code: ExprOrStmt,
}

impl Parser<'_> {
    pub fn parse_defer_stmt(&mut self) -> ParseResult<StmtId> {
        let defer_token = self.bump_expect(&tk::kw_defer)?;
        let code = self.parse_expr_or_stmt(true)?;

        let end_span = match code {
            ExprOrStmt::Expr(expr_id) => self[expr_id].span(),
            ExprOrStmt::Stmt(stmt_id) => self[stmt_id].span(),
        };

        Ok(self.add_stmt(DeferStmt {
            span: defer_token.span.to(end_span),
            code,
        }))
    }
}
