use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::ReturnStmt;
use cool_resolve::FrameId;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ReturnStmtAst {
    pub span: Span,
    pub frame_id: FrameId,
    pub expr: Box<ExprAst>,
}

impl Section for ReturnStmtAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_return_stmt(
        &mut self,
        frame_id: FrameId,
        stmt: &ReturnStmt,
    ) -> AstResult<ReturnStmtAst> {
        let expr_ty_id = self.fn_ret_ty_id();

        let expr = stmt
            .expr
            .as_ref()
            .map(|expr| self.gen_expr(frame_id, expr_ty_id, expr))
            .unwrap_or_else(|| self.implicit_unit_expr(stmt.span().end(), expr_ty_id))?;

        Ok(ReturnStmtAst {
            span: stmt.span,
            frame_id,
            expr: Box::new(expr),
        })
    }
}
