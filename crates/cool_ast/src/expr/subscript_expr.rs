use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::SubscriptExpr;
use cool_resolve::{tys, ExprId, FrameId, ResolveExpr, ResolveExprKind, TyId, ValueTy};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct SubscriptExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub base: Box<ExprAst>,
    pub subscript: Box<ExprAst>,
}

impl Section for SubscriptExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_subscript_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &SubscriptExpr,
    ) -> AstResult<SubscriptExprAst> {
        let base = self.gen_expr(frame_id, tys::INFER, &expr.base)?;
        let subscript = self.gen_expr(frame_id, tys::INFER_SUBSCRIPT, &expr.subscript)?;

        let base_expr = self.resolve[base.expr_id()];
        let subscript_ty_id = self.resolve[subscript.expr_id()].ty_id;

        let (ty_id, kind) = match &self.resolve[base_expr.ty_id].ty {
            ValueTy::ManyPtr(many_ptr_ty) => {
                if subscript_ty_id == tys::USIZE {
                    assert!(!expr.is_mutable);

                    (
                        many_ptr_ty.pointee,
                        ResolveExprKind::Lvalue {
                            is_mutable: many_ptr_ty.is_mutable,
                        },
                    )
                } else {
                    if !many_ptr_ty.is_mutable {
                        assert!(!expr.is_mutable);
                    }

                    (
                        self.resolve.mk_slice(expr.is_mutable, many_ptr_ty.pointee),
                        ResolveExprKind::Rvalue,
                    )
                }
            }
            ValueTy::Slice(slice_ty) => {
                if subscript_ty_id == tys::USIZE {
                    (
                        slice_ty.elem,
                        ResolveExprKind::Lvalue {
                            is_mutable: slice_ty.is_mutable,
                        },
                    )
                } else {
                    if !slice_ty.is_mutable {
                        assert!(!expr.is_mutable);
                    }

                    (
                        self.resolve.mk_slice(expr.is_mutable, slice_ty.elem),
                        ResolveExprKind::Rvalue,
                    )
                }
            }
            ValueTy::Array(array_ty) => {
                if subscript_ty_id == tys::USIZE {
                    (array_ty.elem, base_expr.kind)
                } else {
                    if !matches!(base_expr.kind, ResolveExprKind::Lvalue { is_mutable: true }) {
                        assert!(!expr.is_mutable);
                    }

                    (
                        self.resolve.mk_slice(expr.is_mutable, array_ty.elem),
                        ResolveExprKind::Rvalue,
                    )
                }
            }
            _ => panic!("{:#?} is not subscriptable", base_expr.ty_id),
        };

        let ty_id = self.resolve.resolve_direct_ty_id(ty_id, expected_ty_id)?;
        let expr_id = self.resolve.add_expr(ResolveExpr { ty_id, kind });

        Ok(SubscriptExprAst {
            span: expr.span,
            expr_id,
            base: Box::new(base),
            subscript: Box::new(subscript),
        })
    }
}
