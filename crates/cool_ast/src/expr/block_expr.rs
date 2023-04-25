use crate::{AstGenerator, AstResult};
use cool_parser::BlockExpr;
use cool_resolve::{ExprId, FrameId, TyId};

#[derive(Clone, Debug)]
pub struct BlockExprAst {
    pub expr_id: ExprId,
}

impl AstGenerator<'_> {
    pub fn gen_block_expr(
        &mut self,
        _frame_id: FrameId,
        _expected_ty_id: TyId,
        _expr: &BlockExpr,
    ) -> AstResult<BlockExprAst> {
        todo!()
    }
}
