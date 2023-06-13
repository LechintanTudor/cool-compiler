use crate::{ParseResult, Parser, Ty};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ParenTy {
    pub span: Span,
    pub inner: Box<Ty>,
}

impl Section for ParenTy {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct TupleTy {
    pub span: Span,
    pub elems: Vec<Ty>,
    pub has_trailing_comma: bool,
}

impl Section for TupleTy {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_tuple_ty(&mut self) -> ParseResult<Ty> {
        let open_paren = self.bump_expect(&tk::OPEN_PAREN)?;

        if let Some(close_paren) = self.bump_if_eq(tk::CLOSE_PAREN) {
            return Ok(Ty::Tuple(TupleTy {
                span: open_paren.span.to(close_paren.span),
                elems: vec![],
                has_trailing_comma: false,
            }));
        }

        let first_elem = self.parse_ty()?;

        if let Some(close_paren) = self.bump_if_eq(tk::CLOSE_PAREN) {
            return Ok(Ty::Paren(ParenTy {
                span: open_paren.span.to(close_paren.span),
                inner: Box::new(first_elem),
            }));
        }

        let mut elems = vec![first_elem];

        let (close_paren, has_trailing_comma) = loop {
            self.bump_expect(&tk::COMMA)?;

            if let Some(close_paren) = self.bump_if_eq(tk::CLOSE_PAREN) {
                break (close_paren, true);
            }

            elems.push(self.parse_ty()?);

            if let Some(close_paren) = self.bump_if_eq(tk::CLOSE_PAREN) {
                break (close_paren, false);
            }
        };

        Ok(Ty::Tuple(TupleTy {
            span: open_paren.span.to(close_paren.span),
            elems,
            has_trailing_comma,
        }))
    }
}
