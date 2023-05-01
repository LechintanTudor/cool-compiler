use crate::{Expr, ParseResult, ParseTree, Parser};
use cool_lexer::tokens::tk;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct SubscriptExpr {
    pub span: Span,
    pub base: Box<Expr>,
    pub subscript: Box<Expr>,
}

impl ParseTree for SubscriptExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn continue_parse_subscript_expr(&mut self, base: Box<Expr>) -> ParseResult<SubscriptExpr> {
        let start_token = self.bump_expect(&tk::OPEN_BRACKET)?;
        let subscript = self.parse_expr()?;
        let end_token = self.bump_expect(&tk::CLOSE_BRACKET)?;

        Ok(SubscriptExpr {
            span: start_token.span.to(end_token.span),
            base,
            subscript: Box::new(subscript),
        })
    }
}
