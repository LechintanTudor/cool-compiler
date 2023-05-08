use crate::{AstGenerator, AstResult, BlockExprAst, ExprAst};
use cool_parser::CondBlock;
use cool_resolve::{tys, FrameId, TyId};

#[derive(Clone, Debug)]
pub struct CondBlockAst {
    pub cond: ExprAst,
    pub expr: BlockExprAst,
}

impl AstGenerator<'_> {
    pub fn gen_cond_block(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        block: &CondBlock,
    ) -> AstResult<CondBlockAst> {
        Ok(CondBlockAst {
            cond: self.gen_expr(frame_id, tys::BOOL, &block.cond)?,
            expr: self.gen_block_expr(frame_id, expected_ty_id, &block.expr)?,
        })
    }
}
