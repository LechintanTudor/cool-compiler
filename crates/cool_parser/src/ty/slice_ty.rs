use crate::{ParseResult, Parser, Ty};
use cool_derive::Section;
use cool_lexer::{tk, Token};
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct SliceTy {
    pub span: Span,
    pub is_mutable: bool,
    pub elem_ty: Box<Ty>,
}

impl Parser<'_> {
    pub fn continue_parse_slice_ty(&mut self, open_bracket: Token) -> ParseResult<SliceTy> {
        debug_assert_eq!(open_bracket.kind, tk::open_bracket);
        self.bump_expect(&tk::close_bracket)?;

        let is_mutable = self.bump_if_eq(tk::kw_mut).is_some();
        let elem_ty = self.parse_non_variant_ty()?;

        Ok(SliceTy {
            span: open_bracket.span.to(elem_ty.span()),
            is_mutable,
            elem_ty: Box::new(elem_ty),
        })
    }
}
