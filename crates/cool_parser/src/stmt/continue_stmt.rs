use crate::{ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ContinueStmt {
    pub span: Span,
}

impl Section for ContinueStmt {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_continue_stmt(&mut self) -> ParseResult<ContinueStmt> {
        let token = self.bump_expect(&tk::KW_CONTINUE)?;
        Ok(ContinueStmt { span: token.span })
    }
}
