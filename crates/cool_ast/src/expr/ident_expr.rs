use crate::{AstGenerator, AstResult, TyMismatch};
use cool_parser::IdentExpr;
use cool_resolve::{BindingId, ExprId, FrameId, TyId};

#[derive(Clone, Debug)]
pub struct IdentExprAst {
    pub expr_id: ExprId,
    pub binding_id: BindingId,
}

impl AstGenerator<'_> {
    pub fn gen_ident_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        ident_expr: &IdentExpr,
    ) -> AstResult<IdentExprAst> {
        let binding_id = self
            .resolve
            .resolve_local(frame_id, ident_expr.ident.symbol)?
            .as_binding_id()
            .unwrap();

        let ty_id = self.resolve[binding_id]
            .ty_id
            .resolve_non_inferred(expected_ty_id)
            .ok_or(TyMismatch {
                found_ty: self.resolve[binding_id].ty_id,
                expected_ty: expected_ty_id,
            })?;

        let expr_id = self.resolve.add_expr(ty_id);

        Ok(IdentExprAst {
            expr_id,
            binding_id,
        })
    }
}
