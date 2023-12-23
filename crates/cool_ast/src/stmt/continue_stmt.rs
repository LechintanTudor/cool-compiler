use crate::{ParseResult, Parser, StmtId};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct ContinueStmt {
    pub span: Span,
}

impl Parser<'_> {
    pub fn parse_continue_stmt(&mut self) -> ParseResult<StmtId> {
        let continue_token = self.bump_expect(&tk::kw_continue)?;

        Ok(self.add_stmt(ContinueStmt {
            span: continue_token.span,
        }))
    }
}
