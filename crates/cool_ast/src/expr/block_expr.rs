use crate::{AstGenerator, AstResult, ExprAst, StmtAst, UnitExprAst};
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

        let (found_ty_id, expr) = match block.expr.as_ref() {
            Some(expr) => {
                let expr = self.gen_expr(frame_id, expected_ty_id, expr)?;
                (expr.expr_id().ty_id, expr)
            }
            None => {
                if stmts.last().is_some_and(StmtAst::diverges) {
                    let expr = ExprAst::from(UnitExprAst {
                        span: Span::new(block.span().end() - 1, 0),
                        expr_id: self.implicit_unit_expr_id,
                    });

                    (self.tys().diverge, expr)
                } else {
                    let expr = self.implicit_unit_expr(block.span().end() - 1, expected_ty_id)?;
                    (expr.expr_id().ty_id, expr)
                }
            }
        };

        self.resolve_expr(
            block.span(),
            found_ty_id,
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
