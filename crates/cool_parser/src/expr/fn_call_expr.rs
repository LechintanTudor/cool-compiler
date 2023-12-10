use crate::{ExprId, ParseResult, Parser};
use cool_collections::SmallVec;
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct FnCallExpr {
    pub span: Span,
    pub base: ExprId,
    pub args: SmallVec<ExprId, 4>,
    pub has_trailing_comma: bool,
}

impl Parser<'_> {
    pub fn continue_parse_fn_call_expr(&mut self, base: ExprId) -> ParseResult<ExprId> {
        self.bump_expect(&tk::open_paren)?;
        let mut args = SmallVec::new();

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

        let span = self[base].span().to(close_paren.span);

        Ok(self.add_expr(FnCallExpr {
            span,
            base,
            args,
            has_trailing_comma,
        }))
    }
}
