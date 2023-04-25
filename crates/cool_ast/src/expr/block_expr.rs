use crate::{AstGenerator, AstResult, BlockElemAst, TyMismatch};
use cool_parser::{BlockExpr, BlockElem};
use cool_resolve::{ExprId, FrameId, TyId, tys};

#[derive(Clone, Debug)]
pub struct BlockExprAst {
    pub expr_id: ExprId,
    pub elems: Vec<BlockElemAst>,
}

impl AstGenerator<'_> {
    pub fn gen_block_expr(
        &mut self,
        _frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &BlockExpr,
    ) -> AstResult<BlockExprAst> {
        let Some((_last_elem, other_elems)) = expr.elems.split_first() else {
            tys::UNIT
                .resolve_non_inferred(expected_ty_id)
                .ok_or(TyMismatch {
                    found_ty: tys::UNIT,
                    expected_ty: expected_ty_id,
                })?;

            return Ok(BlockExprAst {
                expr_id: self.resolve.add_expr(tys::UNIT),
                elems: vec![],
            });
        };

        for elem in other_elems.iter() {
            match elem {
                BlockElem::Expr(_expr) => {

                }
                BlockElem::Stmt(_stmt) => {
                    
                }
            }
        }

        todo!()
    }
}
