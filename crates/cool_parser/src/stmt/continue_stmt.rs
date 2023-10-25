use crate::{ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct ContinueStmt {
    pub span: Span,
}

impl Parser<'_> {
    pub fn parse_continue_stmt(&mut self) -> ParseResult<ContinueStmt> {
        Ok(ContinueStmt {
            span: self.bump_expect(&tk::kw_continue)?.span,
        })
    }
}
