use crate::expr::Expr;
use crate::{ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct BreakStmt {
    pub span: Span,
    pub expr: Option<Box<Expr>>,
}

impl Section for BreakStmt {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_break_stmt(&mut self) -> ParseResult<BreakStmt> {
        let start_token = self.bump_expect(&tk::KW_BREAK)?;

        let expr = match self.peek().kind {
            tk::SEMICOLON | tk::COMMA => None,
            _ => Some(self.parse_expr()?),
        };

        let end_span = expr.as_ref().map(Expr::span).unwrap_or(start_token.span);

        Ok(BreakStmt {
            span: start_token.span.to(end_span),
            expr: expr.map(Box::new),
        })
    }
}
