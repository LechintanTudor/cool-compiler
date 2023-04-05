mod decl_stmt;
mod expr_stmt;

pub use self::decl_stmt::*;
pub use self::expr_stmt::*;
use crate::{AstGenerator, ResolveAst, SemanticResult};
use cool_parser::Stmt;
use cool_resolve::resolve::ScopeId;
use cool_resolve::ty::TyId;

#[derive(Clone, Debug)]
pub enum StmtAst {
    Decl(DeclStmtAst),
    Expr(ExprStmtAst),
}

impl ResolveAst for StmtAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: TyId) -> SemanticResult<TyId> {
        match self {
            Self::Decl(decl) => decl.resolve(ast, expected_ty),
            Self::Expr(expr) => expr.resolve(ast, expected_ty),
        }
    }
}

impl From<DeclStmtAst> for StmtAst {
    #[inline]
    fn from(stmt: DeclStmtAst) -> Self {
        Self::Decl(stmt)
    }
}

impl From<ExprStmtAst> for StmtAst {
    #[inline]
    fn from(stmt: ExprStmtAst) -> Self {
        Self::Expr(stmt)
    }
}

impl AstGenerator<'_> {
    pub fn gen_stmt(&mut self, scope_id: ScopeId, stmt: &Stmt) -> StmtAst {
        match stmt {
            Stmt::Decl(stmt) => self.gen_decl_stmt(scope_id, stmt).into(),
            _ => todo!(),
        }
    }
}
