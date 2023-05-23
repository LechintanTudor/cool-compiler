use crate::expr::Expr;
use crate::{ParseResult, Parser};
use cool_lexer::tokens::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ReturnExpr {
    pub span: Span,
    pub expr: Option<Box<Expr>>,
}

impl Section for ReturnExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_return_expr(&mut self) -> ParseResult<ReturnExpr> {
        let start_token = self.bump_expect(&tk::KW_RETURN)?;

        let expr = match self.peek().kind {
            tk::SEMICOLON | tk::COMMA => None,
            _ => Some(self.parse_expr()?),
        };

        let end_span = expr
            .as_ref()
            .map(|expr| expr.span())
            .unwrap_or(start_token.span);

        Ok(ReturnExpr {
            span: start_token.span.to(end_span),
            expr: expr.map(Box::new),
        })
    }
}
