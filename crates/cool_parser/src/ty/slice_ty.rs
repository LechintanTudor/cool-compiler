use crate::{ParseResult, Parser, Ty};
use cool_lexer::tokens::{tk, Token};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct SliceTy {
    pub span: Span,
    pub is_mutable: bool,
    pub elem: Box<Ty>,
}

impl Section for SliceTy {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn continue_parse_slice_ty(&mut self, start_token: Token) -> ParseResult<SliceTy> {
        self.bump_expect(&tk::CLOSE_BRACKET)?;
        let is_mutable = self.bump_if_eq(tk::KW_MUT).is_some();
        let elem = self.parse_ty()?;

        Ok(SliceTy {
            span: start_token.span.to(elem.span()),
            is_mutable,
            elem: Box::new(elem),
        })
    }
}
