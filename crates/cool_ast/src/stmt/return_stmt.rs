use crate::{AstGenerator, AstResult, ExprAst};
use cool_parser::ReturnStmt;
use cool_resolve::FrameId;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ReturnStmtAst {
    pub span: Span,
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
        let expr = stmt
            .expr
            .as_ref()
            .map(|expr| self.gen_expr(frame_id, self.fn_state().ret, expr))
            .transpose()?;

        Ok(ReturnStmtAst {
            span: stmt.span,
            expr: expr.map(Box::new),
        })
    }
}
