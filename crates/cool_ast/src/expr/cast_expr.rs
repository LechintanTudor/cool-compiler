use crate::{AstGenerator, AstResult, AstResultExt, ExprAst, TyError, TyErrorKind};
use cool_parser::CastExpr;
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CastKind {
    PtrToPtr,
    PtrToUsize,
}

#[derive(Clone, Debug)]
pub struct CastExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub base: Box<ExprAst>,
    pub kind: CastKind,
}

impl Section for CastExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_cast_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &CastExpr,
    ) -> AstResult<CastExprAst> {
        let base = self.gen_expr(frame_id, self.tys().infer, &expr.base)?;
        let base_ty_id = self.resolve[base.expr_id()].ty_id;
        let expr_ty_id = self.resolve_ty(frame_id, &expr.ty)?;

        let unsupported_cast = || -> AstResult<CastExprAst> {
            AstResult::error(
                expr.span(),
                TyError {
                    ty_id: base_ty_id,
                    kind: TyErrorKind::UnsupportedCast {
                        to_ty_id: expected_ty_id,
                    },
                },
            )
        };

        let base_value_ty = base_ty_id.shape.as_value().unwrap();
        let expr_value_ty = expr_ty_id.shape.as_value().unwrap();

        let kind = if base_value_ty.is_ptr() || base_value_ty.is_many_ptr() {
            if expr_value_ty.is_ptr() || expr_value_ty.is_many_ptr() {
                CastKind::PtrToPtr
            } else if expr_value_ty.is_usize() {
                CastKind::PtrToUsize
            } else {
                return unsupported_cast();
            }
        } else {
            return unsupported_cast();
        };

        let ty_id = self.resolve_direct_ty_id(expr.span(), expr_ty_id, expected_ty_id)?;

        Ok(CastExprAst {
            span: expr.span(),
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
            base: Box::new(base),
            kind,
        })
    }
}
