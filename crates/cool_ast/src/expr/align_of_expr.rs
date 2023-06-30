use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::AlignOfExpr;
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct AlignOfExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub value: u64,
}

impl Section for AlignOfExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_align_of_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &AlignOfExpr,
    ) -> AstResult<ExprAst> {
        let arg_ty_id = self.resolve_ty(frame_id, &expr.ty)?;
        let value = self.resolve.get_ty_def(arg_ty_id).unwrap().align;

        self.resolve_expr(
            expr.span(),
            self.tys().usize,
            expected_ty_id,
            |resolve, span, ty_id| {
                AlignOfExprAst {
                    span,
                    expr_id: resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    value,
                }
            },
        )
    }
}
