use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::AssignStmt;
use cool_resolve::{tys, FrameId};

#[derive(Clone, Debug)]
pub struct AssignStmtAst {
    pub lvalue: ExprAst,
    pub rvalue: ExprAst,
}

impl AstGenerator<'_> {
    pub fn gen_assign_stmt(
        &mut self,
        frame_id: FrameId,
        assign_stmt: &AssignStmt,
    ) -> AstResult<AssignStmtAst> {
        Ok(AssignStmtAst {
            lvalue: self.gen_expr(frame_id, tys::INFERRED, &assign_stmt.lvalue)?,
            rvalue: self.gen_expr(frame_id, tys::INFERRED, &assign_stmt.rvalue)?,
        })
    }
}
