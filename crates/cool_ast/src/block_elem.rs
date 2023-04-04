use crate::expr::ExprAst;
use crate::stmt::StmtAst;
use crate::{AstGenerator, ResolveAst, SemanticResult, Unify};
use cool_resolve::expr_ty::ExprTyUnifier;
use cool_resolve::ty::{TyId, TyTable};

#[derive(Clone, Debug)]
pub enum BlockElemAst {
    Expr(ExprAst),
    Stmt(StmtAst),
}

impl Unify for BlockElemAst {
    fn unify(&self, unifier: &mut ExprTyUnifier, tys: &mut TyTable) {
        match self {
            Self::Expr(expr) => expr.unify(unifier, tys),
            Self::Stmt(stmt) => stmt.unify(unifier, tys),
        }
    }
}

impl ResolveAst for BlockElemAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: Option<TyId>) -> SemanticResult<TyId> {
        match self {
            Self::Expr(expr) => expr.resolve(ast, expected_ty),
            Self::Stmt(stmt) => stmt.resolve(ast, expected_ty),
        }
    }
}
