use crate::expr::Expr;
use crate::{ParseResult, ParseTree, Parser};
use cool_lexer::tokens::tk;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct ReturnExpr {
    pub span: Span,
    pub expr: Box<Expr>,
}

impl ParseTree for ReturnExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_return_expr(&mut self) -> ParseResult<ReturnExpr> {
        let start_token = self.bump_expect(&tk::KW_RETURN)?;
        let expr = self.parse_expr()?;

        Ok(ReturnExpr {
            span: start_token.span.to(expr.span()),
            expr: Box::new(expr),
        })
    }
}
