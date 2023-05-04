use crate::expr::Expr;
use crate::{ParseResult, ParseTree, Parser};
use cool_lexer::tokens::tk;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct ArrayExpr {
    pub span: Span,
    pub elems: Vec<Expr>,
    pub has_trailing_comma: bool,
}

impl ParseTree for ArrayExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_array_expr(&mut self) -> ParseResult<ArrayExpr> {
        let start_token = self.bump_expect(&tk::OPEN_BRACKET)?;

        if let Some(end_token) = self.bump_if_eq(tk::CLOSE_BRACKET) {
            return Ok(ArrayExpr {
                span: start_token.span.to(end_token.span),
                elems: Default::default(),
                has_trailing_comma: false,
            });
        }

        let mut elems = Vec::<Expr>::new();

        let (end_token, has_trailing_comma) = loop {
            elems.push(self.parse_expr()?);

            if self.bump_if_eq(tk::COMMA).is_some() {
                if let Some(end_token) = self.bump_if_eq(tk::CLOSE_BRACKET) {
                    break (end_token, true);
                }
            } else if let Some(end_token) = self.bump_if_eq(tk::CLOSE_BRACKET) {
                break (end_token, false);
            } else {
                return self.peek_error(&[tk::COMMA, tk::CLOSE_BRACKET]);
            }
        };

        Ok(ArrayExpr {
            span: start_token.span.to(end_token.span),
            elems,
            has_trailing_comma,
        })
    }
}
