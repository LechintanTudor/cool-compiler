use crate::expr::ExprAst;
use crate::stmt::StmtAst;
use crate::{AstGenerator, AstResult, ResolveAst};
use cool_resolve::TyId;

#[derive(Clone, Debug)]
pub enum BlockElemAst {
    Expr(ExprAst),
    Stmt(StmtAst),
}

impl ResolveAst for BlockElemAst {
    fn resolve_exprs(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        match self {
            Self::Expr(expr) => expr.resolve_exprs(ast, expected_ty),
            Self::Stmt(stmt) => stmt.resolve_exprs(ast, expected_ty),
        }
    }
}
