use crate::{AstGenerator, AstResult, AstResultExt, ExprAst};
use cool_resolve::{ExprId, ResolveExpr, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct VariantWrapExprAst {
    pub expr_id: ExprId,
    pub inner: Box<ExprAst>,
}

impl VariantWrapExprAst {
    #[must_use]
    pub fn variant_index(&self) -> u32 {
        let inner_ty_id = self.inner.expr_id().ty_id;

        self.expr_id
            .ty_id
            .get_variant()
            .variants()
            .iter()
            .enumerate()
            .find(|(_, ty_id)| **ty_id == inner_ty_id)
            .map(|(index, _)| index as u32)
            .unwrap()
    }
}

impl Section for VariantWrapExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.inner.span()
    }
}

impl AstGenerator<'_> {
    pub fn continue_gen_variant_wrap_expr(
        &mut self,
        inner: Box<ExprAst>,
        variant_ty_id: TyId,
    ) -> AstResult<VariantWrapExprAst> {
        let inner_ty_id = inner.expr_id().ty_id;

        if !variant_ty_id
            .get_variant()
            .variants()
            .iter()
            .any(|&ty_id| ty_id == inner_ty_id)
        {
            return AstResult::ty_mismatch(inner.span(), inner_ty_id, variant_ty_id);
        }

        Ok(VariantWrapExprAst {
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(variant_ty_id)),
            inner,
        })
    }
}
