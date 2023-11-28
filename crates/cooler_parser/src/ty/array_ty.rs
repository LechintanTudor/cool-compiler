use crate::{ExprId, ParseResult, Parser, TyId};
use cool_derive::Section;
use cool_lexer::{tk, Token};
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct ArrayTy {
    pub span: Span,
    pub len: ExprId,
    pub elem_ty: TyId,
}

#[derive(Clone, Section, Debug)]
pub struct SliceTy {
    pub span: Span,
    pub elem_ty: TyId,
    pub is_mutable: bool,
}

#[derive(Clone, Section, Debug)]
pub struct ManyPtrTy {
    pub span: Span,
    pub pointee_ty: TyId,
    pub is_mutable: bool,
}

impl Parser<'_> {
    pub fn parse_array_or_slice_or_many_ptr_ty(&mut self) -> ParseResult<TyId> {
        let open_bracket = self.bump_expect(&tk::open_bracket)?;

        match self.peek().kind {
            tk::close_bracket => self.continue_parse_slice_ty(open_bracket),
            tk::star => self.continue_parse_many_ptr_ty(open_bracket),
            _ => self.continue_parse_array_ty(open_bracket),
        }
    }

    fn continue_parse_slice_ty(&mut self, open_bracket: Token) -> ParseResult<TyId> {
        debug_assert_eq!(open_bracket.kind, tk::open_bracket);
        self.bump_expect(&tk::close_bracket)?;

        let is_mutable = self.bump_if_eq(tk::kw_mut).is_some();
        let elem_ty = self.parse_non_variant_ty()?;
        let end_span = self.data.tys[elem_ty].span();

        Ok(self.data.tys.push(
            SliceTy {
                span: open_bracket.span.to(end_span),
                elem_ty,
                is_mutable,
            }
            .into(),
        ))
    }

    fn continue_parse_many_ptr_ty(&mut self, open_bracket: Token) -> ParseResult<TyId> {
        debug_assert_eq!(open_bracket.kind, tk::open_bracket);
        self.bump_expect(&tk::star)?;
        self.bump_expect(&tk::close_bracket)?;

        let is_mutable = self.bump_if_eq(tk::kw_mut).is_some();
        let pointee_ty = self.parse_non_variant_ty()?;
        let end_span = self.data.tys[pointee_ty].span();

        Ok(self.data.tys.push(
            ManyPtrTy {
                span: open_bracket.span.to(end_span),
                pointee_ty,
                is_mutable,
            }
            .into(),
        ))
    }

    fn continue_parse_array_ty(&mut self, open_bracket: Token) -> ParseResult<TyId> {
        debug_assert_eq!(open_bracket.kind, tk::open_bracket);

        let len = self.parse_const_expr()?;
        self.bump_expect(&tk::close_bracket)?;

        let elem_ty = self.parse_non_variant_ty()?;
        let end_span = self.data.tys[elem_ty].span();

        Ok(self.data.tys.push(
            ArrayTy {
                span: open_bracket.span.to(end_span),
                len,
                elem_ty,
            }
            .into(),
        ))
    }
}
