use crate::{AstGenerator, AstResult, ExprAst, StmtAst};
use cool_parser::BlockExpr;
use cool_resolve::{tys, ExprId, FrameId, ResolveExpr, TyId};

#[derive(Clone, Debug)]
pub struct BlockExprAst {
    pub expr_id: ExprId,
    pub stmts: Vec<StmtAst>,
    pub expr: Option<Box<ExprAst>>,
}

impl AstGenerator<'_> {
    pub fn gen_block_expr(
        &mut self,
        mut frame_id: FrameId,
        expected_ty_id: TyId,
        block: &BlockExpr,
    ) -> AstResult<BlockExprAst> {
        let mut stmts = Vec::<StmtAst>::new();

        for stmt in block.stmts.iter() {
            let stmt = self.gen_stmt(frame_id, stmt)?;

            if let StmtAst::Decl(decl) = &stmt {
                frame_id = decl.frame_id;
            }

            stmts.push(stmt);
        }

        let (expr, ty_id) = match block.expr.as_ref() {
            Some(expr) => {
                let expr = self
                    .gen_expr(frame_id, expected_ty_id, expr)?
                    .ensure_not_module()?;

                let ty_id = self.resolve[expr.expr_id()].ty_id;
                (Some(expr), ty_id)
            }
            None => {
                let diverges = stmts.last().map(StmtAst::is_return).is_some();

                let ty_id = if diverges && !expected_ty_id.is_inferred() {
                    expected_ty_id
                } else {
                    self.resolve
                        .resolve_direct_ty_id(tys::UNIT, expected_ty_id)?
                };

                (None, ty_id)
            }
        };

        Ok(BlockExprAst {
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
            stmts,
            expr: expr.map(Box::new),
        })
    }
}
