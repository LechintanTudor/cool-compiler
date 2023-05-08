use crate::{AstGenerator, AstResult, BlockExprAst, ExprAst, MissingElseBlock};
use cool_parser::{CondBlock, CondExpr};
use cool_resolve::{tys, ExprId, FrameId, TyId};

#[derive(Clone, Debug)]
pub struct CondBlockAst {
    pub cond: ExprAst,
    pub expr: BlockExprAst,
}

#[derive(Clone, Debug)]
pub struct CondExprAst {
    pub expr_id: ExprId,
    pub if_block: Box<CondBlockAst>,
    pub else_if_blocks: Vec<CondBlockAst>,
    pub else_block: Option<Box<BlockExprAst>>,
}

impl AstGenerator<'_> {
    pub fn gen_cond_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &CondExpr,
    ) -> AstResult<CondExprAst> {
        let if_block = self.gen_cond_block(frame_id, expected_ty_id, &expr.if_block)?;
        let ty_id = self.resolve[if_block.expr.expr_id].ty_id;

        let mut else_if_blocks = Vec::<CondBlockAst>::new();
        for block in expr.else_if_blocks.iter() {
            else_if_blocks.push(self.gen_cond_block(frame_id, ty_id, block)?);
        }

        let else_block = match expr.else_block.as_ref() {
            Some(block) => Some(self.gen_block_expr(frame_id, ty_id, block)?),
            None => {
                if ty_id != tys::UNIT {
                    Err(MissingElseBlock)?;
                }

                None
            }
        };

        Ok(CondExprAst {
            expr_id: self.resolve.add_expr(ty_id, false),
            if_block: Box::new(if_block),
            else_if_blocks,
            else_block: else_block.map(Box::new),
        })
    }

    fn gen_cond_block(
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
