use crate::{AstGenerator, AstResult, CondBlockAst};
use cool_parser::WhileExpr;
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct WhileExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub block: Box<CondBlockAst>,
}

impl Section for WhileExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_while_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &WhileExpr,
    ) -> AstResult<WhileExprAst> {
        let ty_id = self.resolve_direct_ty_id(expr.span(), self.tys().unit, expected_ty_id)?;
        let block = self.gen_cond_block(frame_id, ty_id, &expr.block)?;

        Ok(WhileExprAst {
            span: expr.span,
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
            block: Box::new(block),
        })
    }
}
