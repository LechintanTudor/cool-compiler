use crate::{AstGenerator, AstResult, ExprAst, StmtAst, TyMismatch};
use cool_parser::BlockExpr;
use cool_resolve::{tys, ExprId, FrameId, TyId};

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

        let (expr, ret_ty_id) = match block.expr.as_ref() {
            Some(expr) => {
                let expr = self
                    .gen_expr(frame_id, expected_ty_id, expr)?
                    .ensure_not_module()?;

                let ret_ty_id = self.resolve[expr.id()].ty_id;
                (Some(expr), ret_ty_id)
            }
            None => (None, tys::UNIT),
        };

        ret_ty_id
            .resolve_non_inferred(expected_ty_id)
            .ok_or(TyMismatch {
                found: ret_ty_id,
                expected: expected_ty_id,
            })?;

        Ok(BlockExprAst {
            expr_id: self.resolve.add_expr(ret_ty_id, false),
            stmts,
            expr: expr.map(Box::new),
        })
    }
}
