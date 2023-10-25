use crate::{Expr, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct BreakStmt {
    pub span: Span,
    pub value: Option<Box<Expr>>,
}

impl Parser<'_> {
    pub fn parse_break_stmt(&mut self) -> ParseResult<BreakStmt> {
        let break_token = self.bump_expect(&tk::kw_break)?;

        let value = (!self.peek().kind.is_punct())
            .then(|| self.parse_expr())
            .transpose()?;

        let end_span = value
            .as_ref()
            .map(|value| value.span())
            .unwrap_or(break_token.span);

        Ok(BreakStmt {
            span: break_token.span.to(end_span),
            value: value.map(Box::new),
        })
    }
}
