use crate::{AstGenerator, AstResult, BlockExprAst, ExprAst};
use cool_parser::CondBlock;
use cool_resolve::{tys, FrameId, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct CondBlockAst {
    pub cond: Box<ExprAst>,
    pub expr: Box<BlockExprAst>,
}

impl Section for CondBlockAst {
    #[inline]
    fn span(&self) -> Span {
        self.cond.span().to(self.expr.span())
    }
}

impl AstGenerator<'_> {
    pub fn gen_cond_block(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        block: &CondBlock,
    ) -> AstResult<CondBlockAst> {
        let cond = self.gen_expr(frame_id, tys::BOOL, &block.cond)?;
        let expr = self.gen_block_expr(frame_id, expected_ty_id, &block.expr)?;

        Ok(CondBlockAst {
            cond: Box::new(cond),
            expr: Box::new(expr),
        })
    }
}
