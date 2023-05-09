use crate::{Expr, ParseResult, Parser, UnaryOp};
use cool_lexer::tokens::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct UnaryExpr {
    pub span: Span,
    pub unary_op: UnaryOp,
    pub expr: Box<Expr>,
}

impl Section for UnaryExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_unary_expr(&mut self) -> ParseResult<UnaryExpr> {
        let start_token = self.bump();

        let unary_op = match start_token.kind {
            tk::MINUS => UnaryOp::Minus,
            tk::NOT => UnaryOp::Not,
            tk::AND => UnaryOp::Addr,
            _ => self.error(start_token, &[tk::MINUS, tk::NOT, tk::AND])?,
        };

        let expr = self.parse_expr()?;

        Ok(UnaryExpr {
            span: start_token.span.to(expr.span()),
            unary_op,
            expr: Box::new(expr),
        })
    }
}
