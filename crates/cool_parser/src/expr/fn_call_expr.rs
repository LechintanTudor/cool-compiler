use crate::expr::Expr;
use crate::{ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct FnCallExpr {
    pub span: Span,
    pub base: Box<Expr>,
    pub args: Vec<Expr>,
    pub has_trailing_comma: bool,
}

impl Section for FnCallExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn continue_parse_fn_call_expr(&mut self, base: Box<Expr>) -> ParseResult<FnCallExpr> {
        self.bump_expect(&tk::OPEN_PAREN)?;
        let mut args = Vec::<Expr>::new();

        let (end_token, has_trailing_comma) = match self.peek().kind {
            tk::CLOSE_PAREN => (self.bump_expect(&tk::CLOSE_PAREN)?, false),
            _ => {
                loop {
                    args.push(self.parse_expr()?);

                    if self.bump_if_eq(tk::COMMA).is_some() {
                        if let Some(end_token) = self.bump_if_eq(tk::CLOSE_PAREN) {
                            break (end_token, true);
                        }
                    } else if let Some(end_token) = self.bump_if_eq(tk::CLOSE_PAREN) {
                        break (end_token, false);
                    } else {
                        return self.peek_error(&[tk::COMMA, tk::CLOSE_PAREN]);
                    }
                }
            }
        };

        Ok(FnCallExpr {
            span: base.span().to(end_token.span),
            base,
            args,
            has_trailing_comma,
        })
    }
}
