use crate::{Expr, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct FnCallExpr {
    pub span: Span,
    pub base: Box<Expr>,
    pub args: Vec<Expr>,
    pub has_trailing_comma: bool,
}

impl Parser<'_> {
    pub fn continue_parse_fn_call_expr(&mut self, base: Expr) -> ParseResult<FnCallExpr> {
        self.bump_expect(&tk::open_paren)?;
        let mut args = Vec::<Expr>::new();

        let (close_paren, has_trailing_comma) =
            if let Some(close_paren) = self.bump_if_eq(tk::close_paren) {
                (close_paren, false)
            } else {
                loop {
                    args.push(self.parse_expr()?);

                    if self.bump_if_eq(tk::comma).is_some() {
                        if let Some(close_paren) = self.bump_if_eq(tk::close_paren) {
                            break (close_paren, true);
                        }
                    } else if let Some(close_paren) = self.bump_if_eq(tk::close_paren) {
                        break (close_paren, false);
                    } else {
                        return self.peek_error(&[tk::comma, tk::close_paren]);
                    }
                }
            };

        Ok(FnCallExpr {
            span: base.span().to(close_paren.span),
            base: Box::new(base),
            args,
            has_trailing_comma,
        })
    }
}
