use crate::{AssignToRvalue, AstGenerator, AstResult, ExprAst};
use cool_parser::{AssignOp, AssignStmt};
use cool_resolve::{tys, FrameId};

#[derive(Clone, Debug)]
pub struct AssignStmtAst {
    pub assign_op: AssignOp,
    pub lhs: ExprAst,
    pub rhs: ExprAst,
}

impl AstGenerator<'_> {
    pub fn gen_assign_stmt(
        &mut self,
        frame_id: FrameId,
        assign_stmt: &AssignStmt,
    ) -> AstResult<AssignStmtAst> {
        let lhs = self
            .gen_expr(frame_id, tys::INFERRED, &assign_stmt.lvalue)?
            .ensure_not_module()?;

        if !self.resolve[lhs.id()].is_assignable() {
            Err(AssignToRvalue)?;
        }

        let ty_id = self.resolve[lhs.id()].ty_id;

        let rhs = self
            .gen_expr(frame_id, ty_id, &assign_stmt.rvalue)?
            .ensure_not_module()?;

        Ok(AssignStmtAst {
            assign_op: assign_stmt.assign_op,
            lhs,
            rhs,
        })
    }
}
