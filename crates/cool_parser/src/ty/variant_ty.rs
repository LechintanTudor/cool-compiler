use crate::{ParseResult, Parser, Ty};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct VariantTy {
    pub variant_tys: Vec<Ty>,
}

impl Section for VariantTy {
    fn span(&self) -> Span {
        match self.variant_tys.as_slice() {
            [] => Span::empty(),
            [first] => first.span(),
            [first, .., last] => first.span().to(last.span()),
        }
    }
}

impl Parser<'_> {
    pub fn continue_parse_variant_ty(&mut self, first_ty: Ty) -> ParseResult<VariantTy> {
        let mut variant_tys = vec![first_ty];

        while self.bump_if_eq(tk::or).is_some() {
            variant_tys.push(self.parse_non_variant_ty()?);
        }

        Ok(VariantTy { variant_tys })
    }
}
