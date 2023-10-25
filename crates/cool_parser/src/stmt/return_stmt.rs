use crate::{Expr, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct ReturnStmt {
    pub span: Span,
    pub value: Option<Box<Expr>>,
}

impl Parser<'_> {
    pub fn parse_return_stmt(&mut self) -> ParseResult<ReturnStmt> {
        let return_token = self.bump_expect(&tk::kw_return)?;

        let value = (!self.peek().kind.is_punct())
            .then(|| self.parse_expr())
            .transpose()?;

        let end_span = value
            .as_ref()
            .map(|value| value.span())
            .unwrap_or(return_token.span);

        Ok(ReturnStmt {
            span: return_token.span.to(end_span),
            value: value.map(Box::new),
        })
    }
}
