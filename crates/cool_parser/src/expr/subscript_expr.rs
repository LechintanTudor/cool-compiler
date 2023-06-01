use crate::{Expr, ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct SubscriptExpr {
    pub span: Span,
    pub base: Box<Expr>,
    pub is_mutable: bool,
    pub subscript: Box<Expr>,
}

impl Section for SubscriptExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn continue_parse_subscript_expr(&mut self, base: Box<Expr>) -> ParseResult<SubscriptExpr> {
        let open_bracket = self.bump_expect(&tk::OPEN_BRACKET)?;
        let is_mutable = self.bump_if_eq(tk::KW_MUT).is_some();
        let subscript = self.parse_expr()?;
        let close_bracket = self.bump_expect(&tk::CLOSE_BRACKET)?;

        Ok(SubscriptExpr {
            span: open_bracket.span.to(close_bracket.span),
            base,
            is_mutable,
            subscript: Box::new(subscript),
        })
    }
}
