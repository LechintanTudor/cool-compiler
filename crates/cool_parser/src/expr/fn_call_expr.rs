use crate::expr::Expr;
use crate::{ParseResult, ParseTree, Parser, UnexpectedToken};
use cool_lexer::tokens::{tk, Token};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct FnCallExpr {
    pub span: Span,
    pub fn_expr: Box<Expr>,
    pub arg_exprs: Vec<Expr>,
    pub has_trailing_comma: bool,
}

impl ParseTree for FnCallExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn continue_parse_fn_call_expr(&mut self, fn_expr: Box<Expr>) -> ParseResult<FnCallExpr> {
        self.bump_expect(&[tk::OPEN_PAREN])?;
        let mut arg_exprs = Vec::<Expr>::new();

        let (end_token, has_trailing_comma) = match self.peek().kind {
            tk::CLOSE_PAREN => (self.bump_expect(&[tk::CLOSE_PAREN])?, false),
            _ => loop {
                arg_exprs.push(self.parse_expr()?);

                if self.bump_if_eq(tk::COMMA).is_some() {
                    if let Some(end_token) = self.bump_if_eq(tk::CLOSE_PAREN) {
                        break (end_token, true);
                    }
                } else if let Some(end_token) = self.bump_if_eq(tk::CLOSE_PAREN) {
                    break (end_token, false);
                } else {
                    Err(UnexpectedToken {
                        found: self.peek(),
                        expected: &[tk::COMMA, tk::CLOSE_PAREN],
                    })?
                }
            },
        };

        Ok(FnCallExpr {
            span: fn_expr.span().to(end_token.span),
            fn_expr,
            arg_exprs,
            has_trailing_comma,
        })
    }
}
