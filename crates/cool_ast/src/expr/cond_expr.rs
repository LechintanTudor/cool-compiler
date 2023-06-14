use crate::{AstGenerator, AstResult, BlockExprAst, CondBlockAst};
use cool_parser::CondExpr;
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct CondExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub cond_blocks: Vec<CondBlockAst>,
    pub else_block: Option<Box<BlockExprAst>>,
}

impl CondExprAst {
    #[inline]
    pub fn is_exhaustive(&self) -> bool {
        self.else_block.is_some()
    }
}

impl Section for CondExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_cond_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &CondExpr,
    ) -> AstResult<CondExprAst> {
        let expected_ty_id = (!expr.is_exhaustive())
            .then(|| {
                self.resolve
                    .resolve_direct_ty_id(self.tys().unit, expected_ty_id)
            })
            .transpose()?
            .unwrap_or(expected_ty_id);

        let if_block = self.gen_cond_block(frame_id, expected_ty_id, &expr.if_block)?;
        let ty_id = self.resolve[if_block.expr.expr_id].ty_id;

        let mut cond_blocks = vec![if_block];
        for cond_block in expr.else_if_blocks.iter() {
            cond_blocks.push(self.gen_cond_block(frame_id, ty_id, cond_block)?);
        }

        let else_block = expr
            .else_block
            .as_ref()
            .map(|block| self.gen_block_expr(frame_id, ty_id, block))
            .transpose()?;

        Ok(CondExprAst {
            span: expr.span,
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
            cond_blocks,
            else_block: else_block.map(Box::new),
        })
    }
}
