use crate::{ParseResult, Parser, Ty};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct PtrTy {
    pub span: Span,
    pub is_mutable: bool,
    pub pointee_ty: Box<Ty>,
}

impl Parser<'_> {
    pub fn parse_ptr_ty(&mut self) -> ParseResult<PtrTy> {
        let star_token = self.bump_expect(&tk::star)?;
        let is_mutable = self.bump_if_eq(tk::kw_mut).is_some();
        let pointee_ty = self.parse_non_variant_ty()?;

        Ok(PtrTy {
            span: star_token.span.to(pointee_ty.span()),
            is_mutable,
            pointee_ty: Box::new(pointee_ty),
        })
    }
}
