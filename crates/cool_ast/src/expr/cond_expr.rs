use crate::{AstGenerator, AstResult, BlockExprAst, CondBlockAst};
use cool_parser::CondExpr;
use cool_resolve::{tys, ExprId, FrameId, ResolveExpr, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct CondExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub if_block: Box<CondBlockAst>,
    pub else_if_blocks: Vec<CondBlockAst>,
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
            .then(|| self.resolve.resolve_direct_ty_id(tys::UNIT, expected_ty_id))
            .transpose()?
            .unwrap_or(expected_ty_id);

        let if_block = self.gen_cond_block(frame_id, expected_ty_id, &expr.if_block)?;
        let ty_id = self.resolve[if_block.expr.expr_id].ty_id;

        let else_if_blocks = expr
            .else_if_blocks
            .iter()
            .map(|block| self.gen_cond_block(frame_id, ty_id, block))
            .collect::<Result<Vec<_>, _>>()?;

        let else_block = expr
            .else_block
            .as_ref()
            .map(|block| self.gen_block_expr(frame_id, ty_id, block))
            .transpose()?;

        Ok(CondExprAst {
            span: expr.span,
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
            if_block: Box::new(if_block),
            else_if_blocks,
            else_block: else_block.map(Box::new),
        })
    }
}
