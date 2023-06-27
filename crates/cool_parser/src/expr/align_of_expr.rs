use crate::{ParseResult, Parser, Ty};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct AlignOfExpr {
    pub span: Span,
    pub ty: Box<Ty>,
    pub has_trailing_comma: bool,
}

impl Section for AlignOfExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_align_of_expr(&mut self) -> ParseResult<AlignOfExpr> {
        let start_token = self.bump_expect(&tk::KW_ALIGN_OF)?;
        self.bump_expect(&tk::OPEN_PAREN)?;

        let ty = self.parse_ty()?;

        let (end_token, has_trailing_comma) = match self.bump_if_eq(tk::CLOSE_PAREN) {
            Some(end_token) => (end_token, false),
            None => {
                self.bump_expect(&tk::COMMA)?;
                let end_token = self.bump_expect(&tk::CLOSE_PAREN)?;
                (end_token, true)
            }
        };

        Ok(AlignOfExpr {
            span: start_token.span.to(end_token.span),
            ty: Box::new(ty),
            has_trailing_comma,
        })
    }
}
