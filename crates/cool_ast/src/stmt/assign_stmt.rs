use crate::{AssignToRvalue, AstGenerator, AstResult, ExprAst};
use cool_parser::{AssignOp, AssignStmt};
use cool_resolve::{tys, FrameId};
use cool_span::Section;

#[derive(Clone, Debug)]
pub struct AssignStmtAst {
    pub assign_op: AssignOp,
    pub lhs: ExprAst,
    pub rhs: ExprAst,
}

impl Section for AssignStmtAst {
    #[inline]
    fn span(&self) -> cool_span::Span {
        self.lhs.span().to(self.rhs.span())
    }
}

impl AstGenerator<'_> {
    pub fn gen_assign_stmt(
        &mut self,
        frame_id: FrameId,
        assign_stmt: &AssignStmt,
    ) -> AstResult<AssignStmtAst> {
        let lhs = self
            .gen_expr(frame_id, tys::INFER, &assign_stmt.lhs)?
            .ensure_not_module()?;

        if !self.resolve[lhs.expr_id()].is_assignable() {
            Err(AssignToRvalue)?;
        }

        let ty_id = self.resolve[lhs.expr_id()].ty_id;

        let rhs = self
            .gen_expr(frame_id, ty_id, &assign_stmt.rhs)?
            .ensure_not_module()?;

        Ok(AssignStmtAst {
            assign_op: assign_stmt.assign_op,
            lhs,
            rhs,
        })
    }
}
