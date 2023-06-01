use crate::{ParseResult, Parser, Ty};
use cool_lexer::{tk, Token};
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
    pub fn continue_parse_slice_ty(&mut self, open_bracket: Token) -> ParseResult<SliceTy> {
        debug_assert_eq!(open_bracket.kind, tk::OPEN_BRACKET);
        self.bump_expect(&tk::CLOSE_BRACKET)?;

        let is_mutable = self.bump_if_eq(tk::KW_MUT).is_some();
        let elem = self.parse_ty()?;

        Ok(SliceTy {
            span: open_bracket.span.to(elem.span()),
            is_mutable,
            elem: Box::new(elem),
        })
    }
}
