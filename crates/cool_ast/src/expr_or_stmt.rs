use crate::{AstGenerator, AstResult, ExprAst, StmtAst};
use cool_parser::ExprOrStmt;
use cool_resolve::{tys, FrameId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub enum ExprOrStmtAst {
    Expr(Box<ExprAst>),
    Stmt(Box<StmtAst>),
}

impl Section for ExprOrStmtAst {
    #[inline]
    fn span(&self) -> Span {
        match self {
            Self::Expr(expr) => expr.span(),
            Self::Stmt(stmt) => stmt.span(),
        }
    }
}

impl AstGenerator<'_> {
    pub fn gen_expr_or_stmt(
        &mut self,
        frame_id: FrameId,
        expr_or_stmt: &ExprOrStmt,
    ) -> AstResult<ExprOrStmtAst> {
        match expr_or_stmt {
            ExprOrStmt::Expr(expr) => {
                let expr = self.gen_expr(frame_id, tys::INFER, expr)?;
                Ok(ExprOrStmtAst::Expr(Box::new(expr)))
            }
            ExprOrStmt::Stmt(stmt) => {
                let stmt = self.gen_stmt_kind(frame_id, stmt)?;
                Ok(ExprOrStmtAst::Stmt(Box::new(stmt)))
            }
        }
    }
}
