use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::IndexExpr;
use cool_resolve::{ExprId, FrameId, ResolveExpr, ResolveExprKind, TyId, ValueTy};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct IndexExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub base: Box<ExprAst>,
    pub index: Box<ExprAst>,
}

impl Section for IndexExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_index_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &IndexExpr,
    ) -> AstResult<IndexExprAst> {
        let base = self.gen_expr(frame_id, self.tys().infer, &expr.base)?;
        let index = self.gen_expr(frame_id, self.tys().usize, &expr.index)?;

        let base_expr = base.expr_id();
        let Some(base_ty) = base_expr.ty_id.as_value() else {
            panic!("type is not a value type");
        };

        let (ty_id, kind) = match base_ty {
            ValueTy::Array(array_ty) => (array_ty.elem, base_expr.kind),
            ValueTy::ManyPtr(many_ptr_ty) => {
                (
                    many_ptr_ty.pointee,
                    ResolveExprKind::Lvalue {
                        is_mutable: many_ptr_ty.is_mutable,
                    },
                )
            }
            ValueTy::Slice(slice_ty) => {
                (
                    slice_ty.elem,
                    ResolveExprKind::Lvalue {
                        is_mutable: slice_ty.is_mutable,
                    },
                )
            }
            _ => panic!("{:#?} is not subscriptable", base_expr.ty_id),
        };

        let ty_id = self.resolve_direct_ty_id(expr.span(), ty_id, expected_ty_id)?;
        let expr_id = self.resolve.add_expr(ResolveExpr { ty_id, kind });

        Ok(IndexExprAst {
            span: expr.span,
            expr_id,
            base: Box::new(base),
            index: Box::new(index),
        })
    }
}
