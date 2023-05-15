use crate::{AstGenerator, AstResult, CondBlockAst, TyMismatch};
use cool_parser::WhileExpr;
use cool_resolve::{tys, ExprId, FrameId, ResolveExpr, TyId};

#[derive(Clone, Debug)]
pub struct WhileExprAst {
    pub expr_id: ExprId,
    pub block: Box<CondBlockAst>,
}

impl AstGenerator<'_> {
    pub fn gen_while_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &WhileExpr,
    ) -> AstResult<WhileExprAst> {
        let ty_id = tys::UNIT
            .resolve_non_inferred(expected_ty_id)
            .ok_or(TyMismatch {
                found: tys::UNIT,
                expected: expected_ty_id,
            })?;

        let block = self.gen_cond_block(frame_id, ty_id, &expr.block)?;

        Ok(WhileExprAst {
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
            block: Box::new(block),
        })
    }
}
