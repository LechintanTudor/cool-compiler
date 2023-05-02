use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::{AssignOp, AssignStmt};
use cool_resolve::{tys, FrameId};

#[derive(Clone, Debug)]
pub struct AssignStmtAst {
    pub assign_op: AssignOp,
    pub lvalue: ExprAst,
    pub rvalue: ExprAst,
}

impl AstGenerator<'_> {
    pub fn gen_assign_stmt(
        &mut self,
        frame_id: FrameId,
        assign_stmt: &AssignStmt,
    ) -> AstResult<AssignStmtAst> {
        let lvalue = self
            .gen_expr(frame_id, tys::INFERRED, &assign_stmt.lvalue)?
            .ensure_not_module()?;

        let rvalue = self
            .gen_expr(frame_id, tys::INFERRED, &assign_stmt.rvalue)?
            .ensure_not_module()?;

        Ok(AssignStmtAst {
            assign_op: assign_stmt.assign_op,
            lvalue,
            rvalue,
        })
    }
}
