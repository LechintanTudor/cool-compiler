use crate::expr::ExprAst;
use crate::stmt::StmtAst;
use crate::{AstGenerator, ResolveAst, SemanticResult};
use cool_resolve::ty::TyId;

#[derive(Clone, Debug)]
pub enum BlockElemAst {
    Expr(ExprAst),
    Stmt(StmtAst),
}

impl ResolveAst for BlockElemAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: TyId) -> SemanticResult<TyId> {
        match self {
            Self::Expr(expr) => expr.resolve(ast, expected_ty),
            Self::Stmt(stmt) => stmt.resolve(ast, expected_ty),
        }
    }
}
