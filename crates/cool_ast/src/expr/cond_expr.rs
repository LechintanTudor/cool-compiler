use crate::{AstGenerator, AstResult, CondBlockAst, ExprAst};
use cool_parser::CondExpr;
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct CondExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub cond_blocks: Vec<CondBlockAst>,
    pub else_block: Option<Box<ExprAst>>,
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
    ) -> AstResult<ExprAst> {
        let mut branch_expected_ty_id = expr
            .is_exhaustive()
            .then_some(expected_ty_id)
            .unwrap_or(self.tys().unit);

        let if_block = self.gen_cond_block(frame_id, branch_expected_ty_id, &expr.if_block)?;
        let if_block_ty_id = if_block.expr.expr_id().ty_id;

        if !branch_expected_ty_id.is_value() && if_block_ty_id.is_value() {
            branch_expected_ty_id = if_block_ty_id;
        }

        let mut cond_blocks = vec![if_block];

        for cond_block in expr.else_if_blocks.iter() {
            let else_if_block = self.gen_cond_block(frame_id, branch_expected_ty_id, cond_block)?;
            let else_if_block_ty_id = else_if_block.expr.expr_id().ty_id;

            cond_blocks.push(else_if_block);

            if !branch_expected_ty_id.is_value() && else_if_block_ty_id.is_value() {
                branch_expected_ty_id = else_if_block_ty_id;
            }
        }

        let else_block = expr
            .else_block
            .as_ref()
            .map(|block| self.gen_block_expr(frame_id, branch_expected_ty_id, block))
            .transpose()?;

        self.resolve_expr(
            expr.span(),
            branch_expected_ty_id,
            expected_ty_id,
            |resolve, span, ty_id| {
                CondExprAst {
                    span,
                    expr_id: resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    cond_blocks,
                    else_block: else_block.map(Box::new),
                }
            },
        )
    }
}
