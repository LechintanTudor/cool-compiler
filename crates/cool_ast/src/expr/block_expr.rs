use crate::{AstGenerator, AstResult, ExprAst, StmtAst};
use cool_derive::Section;
use cool_parser::BlockExpr;
use cool_resolve::{tys, ExprId, FrameId, TyId};
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct BlockExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub stmts: Vec<StmtAst>,
    pub end_expr: Option<Box<ExprAst>>,
}

impl AstGenerator<'_> {
    pub fn gen_block_expr(
        &mut self,
        expr: &BlockExpr,
        frame_id: FrameId,
        expected_ty_id: TyId,
    ) -> AstResult<ExprAst> {
        let frame_id = self.context.add_frame(frame_id);

        let stmts = expr
            .stmts
            .iter()
            .map(|stmt| self.gen_stmt(&stmt.stmt, frame_id))
            .collect::<Result<Vec<_>, _>>()?;

        let end_expr = expr
            .end_expr
            .as_ref()
            .map(|expr| self.gen_expr(expr, frame_id, expected_ty_id))
            .transpose()?;

        let ty_id = end_expr
            .as_ref()
            .map(|expr| self.context[expr.expr_id()].ty_id)
            .unwrap_or(tys::unit);

        self.resolve_expr(expr.span, ty_id, expected_ty_id, |context, span, ty_id| {
            BlockExprAst {
                span,
                expr_id: context.add_rvalue_expr(ty_id),
                stmts,
                end_expr: end_expr.map(Box::new),
            }
        })
    }
}
