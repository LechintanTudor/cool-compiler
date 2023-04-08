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
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        match self {
            Self::Expr(expr) => expr.resolve(ast, expected_ty),
            Self::Stmt(stmt) => stmt.resolve(ast, expected_ty),
        }
    }
}
