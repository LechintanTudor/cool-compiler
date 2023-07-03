use crate::{
    AstGenerator, AstResult, AstResultExt, ExprAst, TyError, TyErrorKind, VariantWrapExprAst,
};
use cool_parser::CastExpr;
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyId, ValueTy};
use cool_span::{Section, Span};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CastKind {
    PtrToPtr,
    PtrToUsize,
    TupleToSlice,
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
    ) -> AstResult<ExprAst> {
        let base = self.gen_expr(frame_id, self.tys().infer, &expr.base)?;
        let base_ty_id = base.expr_id().ty_id;
        let expr_ty_id = self.resolve_ty(frame_id, &expr.ty)?;

        if base_ty_id == expr_ty_id {
            return Ok(base);
        }

        let unsupported_cast = || -> AstResult<ExprAst> {
            AstResult::error(
                expr.span(),
                TyError {
                    ty_id: base_ty_id,
                    kind: TyErrorKind::UnsupportedCast {
                        to_ty_id: expr_ty_id,
                    },
                },
            )
        };

        if let ValueTy::Variant(variant_ty) = expr_ty_id.get_value() {
            if !variant_ty.has_variant(base_ty_id) {
                return unsupported_cast();
            }

            return self.resolve_expr(expr.span(), expr_ty_id, expected_ty_id, |resolve, _, _| {
                VariantWrapExprAst {
                    expr_id: resolve.add_expr(ResolveExpr::rvalue(expr_ty_id)),
                    inner: Box::new(base),
                }
            });
        }

        let kind = match base_ty_id.get_value() {
            ValueTy::Ptr(_) | ValueTy::ManyPtr(_) => {
                if expr_ty_id.is_ptr() || expr_ty_id.is_many_ptr() {
                    CastKind::PtrToPtr
                } else if expr_ty_id.is_usize() {
                    CastKind::PtrToUsize
                } else {
                    return unsupported_cast();
                }
            }
            ValueTy::Tuple(tuple_ty) => {
                match (tuple_ty.elems(), expr_ty_id.get_value()) {
                    ([many_ptr_ty_id, len_ty_id], ValueTy::Slice(slice_ty)) => {
                        let expected_many_ptr_ty_id =
                            self.resolve.mk_many_ptr(slice_ty.elem, slice_ty.is_mutable);

                        if *many_ptr_ty_id == expected_many_ptr_ty_id && len_ty_id.is_usize() {
                            CastKind::TupleToSlice
                        } else {
                            return unsupported_cast();
                        }
                    }
                    _ => return unsupported_cast(),
                }
            }
            _ => return unsupported_cast(),
        };

        self.resolve_expr(
            expr.span(),
            expr_ty_id,
            expected_ty_id,
            |resolve, span, ty_id| {
                CastExprAst {
                    span,
                    expr_id: resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    base: Box::new(base),
                    kind,
                }
            },
        )
    }
}
