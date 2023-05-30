use crate::{AstGenerator, AstResult};
use cool_parser::RangeExpr;
use cool_resolve::{tys, ExprId, FrameId, ResolveExpr, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub enum RangeExprKindAst {
    Full,
}

#[derive(Clone, Debug)]
pub struct RangeExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub kind: RangeExprKindAst,
}

impl Section for RangeExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_range_expr(
        &mut self,
        _frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &RangeExpr,
    ) -> AstResult<RangeExprAst> {
        let ty_id = self
            .resolve
            .resolve_direct_ty_id(tys::RANGE_FULL, expected_ty_id)?;

        Ok(RangeExprAst {
            span: expr.span,
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
            kind: RangeExprKindAst::Full,
        })
    }
}
