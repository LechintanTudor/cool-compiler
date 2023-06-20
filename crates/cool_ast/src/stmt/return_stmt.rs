use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::ReturnStmt;
use cool_resolve::FrameId;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ReturnStmtAst {
    pub span: Span,
    pub frame_id: FrameId,
    pub expr: Option<Box<ExprAst>>,
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
            .transpose()?;

        Ok(ReturnStmtAst {
            span: stmt.span,
            frame_id,
            expr: expr.map(Box::new),
        })
    }
}
