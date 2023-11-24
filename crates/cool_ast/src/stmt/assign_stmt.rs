use crate::{AssignOp, AstGenerator, ExprAst, SemanticError, SpannedAstError, SpannedAstResult};
use cool_parser::AssignStmt;
use cool_resolve::{tys, FrameId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct AssignStmtAst {
    pub lhs: Box<ExprAst>,
    pub op: AssignOp,
    pub rhs: Box<ExprAst>,
}

impl Section for AssignStmtAst {
    #[inline]
    fn span(&self) -> Span {
        self.lhs.span().to(self.rhs.span())
    }
}

impl AstGenerator<'_> {
    pub fn gen_assign_stmt(
        &mut self,
        stmt: &AssignStmt,
        frame_id: FrameId,
    ) -> SpannedAstResult<AssignStmtAst> {
        let op: AssignOp = stmt.op.into();

        let (lhs, rhs) = match op {
            AssignOp::Eq => {
                let lhs = self.gen_expr(&stmt.lhs, frame_id, tys::infer)?;
                let lhs_ty_id = self.context[lhs.expr_id()].ty_id;
                let rhs = self.gen_expr(&stmt.rhs, frame_id, lhs_ty_id)?;
                (lhs, rhs)
            }
            AssignOp::Add | AssignOp::Sub | AssignOp::Mul | AssignOp::Div | AssignOp::Rem => {
                let lhs = self.gen_expr(&stmt.lhs, frame_id, tys::infer_number)?;
                let lhs_ty_id = self.context[lhs.expr_id()].ty_id;
                let rhs = self.gen_expr(&stmt.rhs, frame_id, lhs_ty_id)?;
                (lhs, rhs)
            }
            AssignOp::Or | AssignOp::And | AssignOp::Xor => {
                let lhs = self.gen_expr(&stmt.lhs, frame_id, tys::infer_int_or_bool)?;
                let lhs_ty_id = self.context[lhs.expr_id()].ty_id;
                let rhs = self.gen_expr(&stmt.rhs, frame_id, lhs_ty_id)?;
                (lhs, rhs)
            }
            AssignOp::Shl | AssignOp::Shr => {
                let lhs = self.gen_expr(&stmt.lhs, frame_id, tys::infer_int)?;
                let rhs = self.gen_expr(&stmt.rhs, frame_id, tys::infer_int)?;
                (lhs, rhs)
            }
        };

        if !self.context[lhs.expr_id()].kind.is_assignable() {
            return Err(SpannedAstError::new(
                stmt.span(),
                SemanticError::ExprNotAssignable {
                    expr_id: lhs.expr_id(),
                },
            ));
        }

        Ok(AssignStmtAst {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        })
    }
}
