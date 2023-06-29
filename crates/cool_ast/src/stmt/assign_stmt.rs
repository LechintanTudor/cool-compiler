use crate::{AstGenerator, AstResult, AstResultExt, ExprAst, ExprError};
use cool_parser::{AssignOp, AssignStmt};
use cool_resolve::FrameId;
use cool_span::Section;

#[derive(Clone, Debug)]
pub struct AssignStmtAst {
    pub assign_op: AssignOp,
    pub lhs: Box<ExprAst>,
    pub rhs: Box<ExprAst>,
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
        stmt: &AssignStmt,
    ) -> AstResult<AssignStmtAst> {
        let lhs = self.gen_expr(frame_id, self.tys().infer, &stmt.lhs)?;
        if !lhs.expr_id().is_assignable() {
            return AstResult::error(stmt.span(), ExprError::NotAssignable);
        }

        let ty_id = lhs.expr_id().ty_id;
        let rhs = self.gen_expr(frame_id, ty_id, &stmt.rhs)?;

        Ok(AssignStmtAst {
            assign_op: stmt.assign_op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
}
