use crate::{AstGenerator, AstResult, ExprAst, TyNotPointer};
use cool_parser::DerefExpr;
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyId, TyKind};

#[derive(Clone, Debug)]
pub struct DerefExprAst {
    pub expr_id: ExprId,
    pub expr: Box<ExprAst>,
}

impl AstGenerator<'_> {
    pub fn gen_deref_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        deref_expr: &DerefExpr,
    ) -> AstResult<DerefExprAst> {
        let expr = self.gen_expr(frame_id, expected_ty_id, &deref_expr.expr)?;
        let expr_ty_id = self.resolve[expr.id()].ty_id;

        let TyKind::Pointer(pointer_ty) = self.resolve[expr_ty_id].kind else {
            Err(TyNotPointer)?
        };

        let ty_id = self
            .resolve
            .resolve_direct_ty_id(pointer_ty.pointee, expected_ty_id)?;

        Ok(DerefExprAst {
            expr_id: self
                .resolve
                .add_expr(ResolveExpr::lvalue(ty_id, pointer_ty.is_mutable)),
            expr: Box::new(expr),
        })
    }
}
