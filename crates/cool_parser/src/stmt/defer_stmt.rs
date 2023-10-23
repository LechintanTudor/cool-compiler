use crate::{ParseResult, Parser, Stmt};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct DeferStmt {
    pub span: Span,
    pub body: Box<Stmt>,
}

impl Parser<'_> {
    pub fn parse_defer_stmt(&mut self) -> ParseResult<DeferStmt> {
        let defer_token = self.bump_expect(&tk::kw_defer)?;
        let body = Stmt::from(self.parse_expr_or_stmt()?);

        Ok(DeferStmt {
            span: defer_token.span.to(body.span()),
            body: Box::new(body),
        })
    }
}
