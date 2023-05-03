use crate::{AstGenerator, AstResult, BlockElemAst, StmtAst, TyMismatch};
use cool_parser::{BlockElem, BlockExpr};
use cool_resolve::{tys, ExprId, FrameId, TyId};

#[derive(Clone, Debug)]
pub struct BlockExprAst {
    pub expr_id: ExprId,
    pub elems: Vec<BlockElemAst>,
}

impl AstGenerator<'_> {
    pub fn gen_block_expr(
        &mut self,
        mut frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &BlockExpr,
    ) -> AstResult<BlockExprAst> {
        let mut elems = Vec::<BlockElemAst>::new();

        for (i, elem) in expr.elems.iter().enumerate() {
            let expected_ty_id = if i + 1 == expr.elems.len() {
                expected_ty_id
            } else {
                tys::INFERRED
            };

            let elem: BlockElemAst = match elem {
                BlockElem::Expr(expr) => {
                    self.gen_expr(frame_id, expected_ty_id, expr)?
                        .ensure_not_module()?
                        .into()
                }
                BlockElem::Stmt(stmt) => {
                    let stmt = self.gen_stmt(frame_id, stmt)?;

                    if let StmtAst::Decl(decl) = &stmt {
                        frame_id = decl.frame_id;
                    }

                    stmt.into()
                }
            };

            elems.push(elem);
        }

        let ret_ty_id = match elems.last() {
            Some(BlockElemAst::Expr(last_expr)) => self.resolve[last_expr.id()],
            _ => tys::UNIT,
        };

        ret_ty_id
            .resolve_non_inferred(expected_ty_id)
            .ok_or(TyMismatch {
                found: ret_ty_id,
                expected: expected_ty_id,
            })?;

        let expr_id = self.resolve.add_expr(ret_ty_id);
        Ok(BlockExprAst { expr_id, elems })
    }
}
