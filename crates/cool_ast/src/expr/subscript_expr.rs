use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::SubscriptExpr;
use cool_resolve::{tys, ExprId, FrameId, ResolveExpr, TyId, TyKind};

#[derive(Clone, Debug)]
pub struct SubscriptExprAst {
    pub expr_id: ExprId,
    pub base: Box<ExprAst>,
    pub index: Box<ExprAst>,
}

impl AstGenerator<'_> {
    pub fn gen_subscript_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &SubscriptExpr,
    ) -> AstResult<SubscriptExprAst> {
        let base = self.gen_expr(frame_id, tys::INFER, &expr.base)?;
        let index = self.gen_expr(frame_id, tys::USIZE, &expr.index)?;

        let base_resolve_expr = self.resolve[base.expr_id()];

        let TyKind::Array(array_ty) = self.resolve[base_resolve_expr.ty_id].kind else {
            panic!("ty not array");
        };

        let ty_id = self
            .resolve
            .resolve_direct_ty_id(array_ty.elem, expected_ty_id)?;

        Ok(SubscriptExprAst {
            expr_id: self.resolve.add_expr(ResolveExpr {
                ty_id,
                kind: base_resolve_expr.kind,
            }),
            base: Box::new(base),
            index: Box::new(index),
        })
    }
}
