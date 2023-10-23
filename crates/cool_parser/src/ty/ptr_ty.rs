use crate::{ParseResult, Parser, Ty};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct PtrTy {
    pub span: Span,
    pub is_mutable: bool,
    pub pointee: Box<Ty>,
}

impl Parser<'_> {
    pub fn parse_ptr_ty(&mut self) -> ParseResult<PtrTy> {
        let star_token = self.bump_expect(&tk::star)?;
        let is_mutable = self.bump_if_eq(tk::kw_mut).is_some();
        let pointee = self.parse_ty()?;

        Ok(PtrTy {
            span: star_token.span.to(pointee.span()),
            is_mutable,
            pointee: Box::new(pointee),
        })
    }
}
