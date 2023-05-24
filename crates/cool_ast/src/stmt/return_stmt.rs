use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::ReturnStmt;
use cool_resolve::FrameId;

#[derive(Clone, Debug)]
pub struct ReturnStmtAst {
    pub expr: Option<Box<ExprAst>>,
}

impl AstGenerator<'_> {
    pub fn gen_return_stmt(
        &mut self,
        frame_id: FrameId,
        stmt: &ReturnStmt,
    ) -> AstResult<ReturnStmtAst> {
        let expr = stmt
            .expr
            .as_ref()
            .map(|expr| self.gen_expr(frame_id, self.fn_state().ret, expr))
            .transpose()?;

        Ok(ReturnStmtAst {
            expr: expr.map(Box::new),
        })
    }
}
