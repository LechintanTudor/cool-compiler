use crate::{Expr, IndexExpr, ParseResult, Parser, RangeExpr, RangeExprKind};
use cool_lexer::tk;

impl Parser<'_> {
    pub fn continue_parse_index_or_range_expr(&mut self, base: Expr) -> ParseResult<Expr> {
        let open_bracket = self.bump_expect(&tk::open_bracket)?;
        let is_mutable = self.bump_if_eq(tk::kw_mut).is_some();

        let from_expr = (self.peek().kind != tk::dot_dot)
            .then(|| self.parse_expr())
            .transpose()?;

        #[allow(clippy::unnecessary_unwrap)]
        if !is_mutable && from_expr.is_some() {
            if let Some(close_bracket) = self.bump_if_eq(tk::close_bracket) {
                return Ok(IndexExpr {
                    span: open_bracket.span.to(close_bracket.span),
                    base: Box::new(base),
                    index: Box::new(from_expr.unwrap()),
                }
                .into());
            }
        }

        self.bump_expect(&tk::dot_dot)?;

        let to_expr = (self.peek().kind != tk::close_bracket)
            .then(|| self.parse_expr())
            .transpose()?;

        let close_bracket = self.bump_expect(&tk::close_bracket)?;

        let kind = match (from_expr, to_expr) {
            (None, None) => RangeExprKind::Full,
            (Some(from), None) => RangeExprKind::From(Box::new(from)),
            (None, Some(to)) => RangeExprKind::To(Box::new(to)),
            (Some(from), Some(to)) => RangeExprKind::FromTo(Box::new((from, to))),
        };

        Ok(RangeExpr {
            span: open_bracket.span.to(close_bracket.span),
            base: Box::new(base),
            is_mutable,
            kind,
        }
        .into())
    }
}
