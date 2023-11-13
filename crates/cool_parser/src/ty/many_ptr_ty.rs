use crate::{ParseResult, Parser, Ty};
use cool_derive::Section;
use cool_lexer::{tk, Token};
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct ManyPtrTy {
    pub span: Span,
    pub is_mutable: bool,
    pub pointee_ty: Box<Ty>,
}

impl Parser<'_> {
    pub fn continue_parse_many_ptr_ty(&mut self, open_bracket: Token) -> ParseResult<ManyPtrTy> {
        debug_assert_eq!(open_bracket.kind, tk::open_bracket);
        self.bump_expect(&tk::star)?;
        self.bump_expect(&tk::close_bracket)?;

        let is_mutable = self.bump_if_eq(tk::kw_mut).is_some();
        let pointee_ty = self.parse_non_variant_ty()?;

        Ok(ManyPtrTy {
            span: open_bracket.span.to(pointee_ty.span()),
            is_mutable,
            pointee_ty: Box::new(pointee_ty),
        })
    }
}
