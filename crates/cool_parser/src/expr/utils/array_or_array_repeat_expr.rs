use crate::{ArrayExpr, ArrayRepeatExpr, Expr, ParseResult, Parser};
use cool_lexer::tk;

impl Parser<'_> {
    pub fn parse_array_or_array_repeat_expr(&mut self) -> ParseResult<Expr> {
        let open_bracket = self.bump_expect(&tk::open_bracket)?;

        if let Some(close_bracket) = self.bump_if_eq(tk::close_bracket) {
            return Ok(ArrayExpr {
                span: open_bracket.span.to(close_bracket.span),
                values: vec![],
                has_trailing_comma: false,
            }
            .into());
        }

        let first_value = self.parse_expr()?;

        if self.bump_if_eq(tk::semicolon).is_some() {
            let len = self.parse_array_len()?;
            let close_bracket = self.bump_expect(&tk::close_bracket)?;

            return Ok(ArrayRepeatExpr {
                span: open_bracket.span.to(close_bracket.span),
                len,
                value: Box::new(first_value),
            }
            .into());
        }

        let mut values = vec![first_value];

        let (close_bracket, has_trailing_comma) =
            if let Some(close_bracket) = self.bump_if_eq(tk::close_bracket) {
                (close_bracket, false)
            } else {
                loop {
                    self.bump_expect(&tk::comma)?;

                    if let Some(close_bracket) = self.bump_if_eq(tk::close_bracket) {
                        break (close_bracket, true);
                    }

                    values.push(self.parse_expr()?);

                    if let Some(close_bracket) = self.bump_if_eq(tk::close_bracket) {
                        break (close_bracket, false);
                    }
                }
            };

        Ok(ArrayExpr {
            span: open_bracket.span.to(close_bracket.span),
            values,
            has_trailing_comma,
        }
        .into())
    }
}
