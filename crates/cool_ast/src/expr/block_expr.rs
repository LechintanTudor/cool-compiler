use crate::{AstGenerator, AstResult, ExprAst, StmtAst};
use cool_parser::BlockExpr;
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct BlockExprAst {
    pub span: Span,
    pub first_frame_id: FrameId,
    pub last_frame_id: FrameId,
    pub expr_id: ExprId,
    pub stmts: Vec<StmtAst>,
    pub expr: Box<ExprAst>,
}

impl Section for BlockExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_block_expr(
        &mut self,
        mut frame_id: FrameId,
        expected_ty_id: TyId,
        block: &BlockExpr,
    ) -> AstResult<ExprAst> {
        let first_frame_id = frame_id;
        let mut stmts = Vec::<StmtAst>::new();

        for stmt in block.stmts.iter() {
            let stmt = self.gen_stmt(frame_id, stmt)?;

            if let Some(new_frame_id) = stmt.get_new_frame_id() {
                frame_id = new_frame_id;
            }

            stmts.push(stmt);
        }

        let expr = block
            .expr
            .as_ref()
            .map(|expr| self.gen_expr(frame_id, expected_ty_id, expr))
            .unwrap_or_else(|| self.implicit_unit_expr(block.span().end() - 1, expected_ty_id))?;

        self.resolve_expr(
            block.span(),
            expr.expr_id().ty_id,
            expected_ty_id,
            |resolve, span, ty_id| {
                BlockExprAst {
                    span,
                    first_frame_id,
                    last_frame_id: frame_id,
                    expr_id: resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    stmts,
                    expr: Box::new(expr),
                }
            },
        )
    }
}
